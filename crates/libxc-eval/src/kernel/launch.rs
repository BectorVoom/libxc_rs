//! Kernel launch infrastructure for CubeCL compute kernels.
//!
//! Provides reusable utilities for launching CubeCL kernels:
//! - Backend selection (CPU default in Phase 2)
//! - Buffer management (upload, zero-init allocation, readback)
//! - CubeCount/CubeDim calculation for 1D grid dispatch
//!
//! Every kernel evaluation goes through this infrastructure.

use cubecl::cpu::{CpuDevice, CpuRuntime};
use cubecl::client::ComputeClient;
use cubecl::prelude::*;

/// Workgroup size for 1D compute kernels.
/// 256 threads per workgroup is the standard choice for compute shaders.
const WORKGROUP_SIZE: u32 = 256;

/// Calculate CubeCount and CubeDim for a 1D kernel over `np` grid points.
///
/// Uses 256 threads per workgroup (standard for compute kernels).
/// The CubeCount rounds up to ensure all elements are covered;
/// kernels MUST include a bounds check (`if ip >= output.len() { return; }`)
/// to handle excess threads in the last workgroup (per T-02-07).
pub fn calculate_launch_config(np: usize) -> (CubeCount, CubeDim) {
    let cube_dim = CubeDim::new_1d(WORKGROUP_SIZE);
    if np == 0 {
        return (CubeCount::Static(0, 1, 1), cube_dim);
    }
    let num_cubes = (np as u32).div_ceil(WORKGROUP_SIZE);
    (CubeCount::new_1d(num_cubes), cube_dim)
}

/// Create a CubeCL CPU compute client.
///
/// This is the default backend for Phase 2. GPU backends (CUDA, HIP, WGPU)
/// will be added behind feature gates in Phase 7.
pub fn cpu_client() -> ComputeClient<CpuRuntime> {
    let device = CpuDevice;
    CpuRuntime::client(&device)
}

// ---- Buffer helpers ---------------------------------------------------------
//
// 11-12 (G-2 / G4): the buffer helpers are generic over the element type so the
// f32 oracle leg can upload/read f32 device buffers. The kernels are
// `#[cube] fn k<F: Float>(...)`; the launch infers F from the argument element
// types (ArrayArg element + bare scalars + dt/zt). The f64 entry points below
// preserve the existing API (266 dispatch call sites pass f64 and are
// unchanged); the f32 path calls the `_generic` forms with `F = f32`.
//
// Bound is `bytemuck::Pod` only: buffer create/read move raw bytes
// (`create_from_slice`/`read_one` are byte-oriented); the CubeCL element type
// is fixed at the kernel-launch ArrayArg, not here.

/// Upload a slice of any Pod element type to device memory, returning a handle.
/// Bit patterns are preserved exactly through the round-trip (T-02-05).
pub fn create_input_buffer_generic<F: bytemuck::Pod>(
    client: &ComputeClient<CpuRuntime>,
    data: &[F],
) -> cubecl::server::Handle {
    client.create_from_slice(bytemuck::cast_slice(data))
}

/// Create a zero-initialized output buffer of `n` elements of `F`.
/// Zero-init is CRITICAL (T-02-06): kernels use `+=` accumulation.
///
/// Host-zeros path, retained for element types the `#[cube]` zero-fill kernel
/// is not instantiated for. The f64 entry point below zeroes on device instead.
pub fn create_zero_output_buffer_generic<F: bytemuck::Pod>(
    client: &ComputeClient<CpuRuntime>,
    n: usize,
) -> cubecl::server::Handle {
    let zeros = vec![<F as bytemuck::Zeroable>::zeroed(); n];
    client.create_from_slice(bytemuck::cast_slice(&zeros))
}

/// Zero an f64 device buffer in place.
#[cube(launch_unchecked)]
fn zero_fill_f64(out: &mut Array<f64>) {
    let i = ABSOLUTE_POS;
    if i < out.len() {
        out[i] = 0.0;
    }
}

/// Read a buffer of `F` elements back from device to host.
/// Bit patterns are preserved exactly through the round-trip (T-02-05).
pub fn read_output_buffer_generic<F: bytemuck::Pod>(
    client: &ComputeClient<CpuRuntime>,
    handle: cubecl::server::Handle,
    _n: usize,
) -> Vec<F> {
    let bytes = client
        .read_one(handle)
        .expect("read_one failed during output buffer read-back");
    bytemuck::cast_slice(&bytes).to_vec()
}

/// Read an f64 device buffer straight into `dst`, with no intermediate `Vec`.
///
/// `read_output_buffer` allocated a `Vec` via `.to_vec()` and the caller then
/// copied that into its own slice, so every output was copied twice and
/// allocated once for nothing. `read_one` hands back owned `Bytes` already, so
/// one `copy_from_slice` into the caller's buffer is all that is needed.
///
/// Returns `Err(actual_element_count)` when the device buffer length does not
/// match `dst`, letting the caller raise `OutputBufferSizeMismatch`.
pub fn read_output_buffer_into(
    client: &ComputeClient<CpuRuntime>,
    handle: cubecl::server::Handle,
    dst: &mut [f64],
) -> Result<(), usize> {
    let bytes = client
        .read_one(handle)
        .expect("read_one failed during output buffer read-back");
    let src: &[f64] = bytemuck::cast_slice(&bytes);
    if src.len() != dst.len() {
        return Err(src.len());
    }
    dst.copy_from_slice(src);
    Ok(())
}

/// Upload an f64 slice to device memory, returning a handle.
pub fn create_input_buffer(
    client: &ComputeClient<CpuRuntime>,
    data: &[f64],
) -> cubecl::server::Handle {
    create_input_buffer_generic(client, data)
}

/// Create a zero-initialized output buffer of `n` f64 elements.
///
/// Allocates uninitialised device memory and zeroes it with a device-side
/// kernel, rather than building an `n`-element zero `Vec` on the host and
/// uploading it. The old path allocated and then moved `n * 8` bytes across the
/// host/device boundary for every output field of every evaluation; profiling
/// the GGA vxc path at n = 1e6 attributed 2.2 ms of a 6.2 ms per-call overhead
/// to those uploads and a further 0.5 ms to the host allocations. Zeroing in
/// place writes the buffer once and transfers nothing.
pub fn create_zero_output_buffer(
    client: &ComputeClient<CpuRuntime>,
    n: usize,
) -> cubecl::server::Handle {
    let handle = client.empty(n * core::mem::size_of::<f64>());
    if n > 0 {
        let (cube_count, cube_dim) = calculate_launch_config(n);
        unsafe {
            zero_fill_f64::launch_unchecked::<CpuRuntime>(
                client,
                cube_count,
                cube_dim,
                ArrayArg::from_raw_parts(handle.clone(), n),
            );
        }
    }
    handle
}

/// Read an f64 buffer back from device to host.
pub fn read_output_buffer(
    client: &ComputeClient<CpuRuntime>,
    handle: cubecl::server::Handle,
    n: usize,
) -> Vec<f64> {
    read_output_buffer_generic::<f64>(client, handle, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Extract the (x, y, z) components from a CubeCount.
    /// CubeCount does not implement PartialEq, so we destructure for assertions.
    fn cube_count_xyz(cc: &CubeCount) -> (u32, u32, u32) {
        match cc {
            CubeCount::Static(x, y, z) => (*x, *y, *z),
            CubeCount::Dynamic(_) => panic!("unexpected dynamic CubeCount in test"),
        }
    }

    /// Trivial identity kernel: output[i] = input[i].
    /// Validates the full launch pipeline: create client -> upload input ->
    /// allocate output -> launch kernel -> read results.
    /// Per T-02-07: bounds check at top prevents out-of-bounds writes
    /// when CubeCount rounds up.
    #[cube(launch_unchecked)]
    fn identity_kernel(input: &Array<f64>, output: &mut Array<f64>) {
        let ip = ABSOLUTE_POS;
        if ip < output.len() {
            output[ip] = input[ip];
        }
    }

    /// Helper to launch identity kernel through the full infrastructure.
    fn run_identity(data: &[f64]) -> Vec<f64> {
        let client = cpu_client();
        let n = data.len();
        let input_handle = create_input_buffer(&client, data);
        let output_handle = create_zero_output_buffer(&client, n);
        let (cube_count, cube_dim) = calculate_launch_config(n);

        unsafe {
            identity_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                cube_count,
                cube_dim,
                ArrayArg::from_raw_parts(input_handle, n),
                ArrayArg::from_raw_parts(output_handle.clone(), n),
            );
        }

        read_output_buffer(&client, output_handle, n)
    }

    // --- Launch config tests ---

    #[test]
    fn test_launch_config_1000() {
        let (cube_count, cube_dim) = calculate_launch_config(1000);
        // (1000 + 255) / 256 = 4
        assert_eq!(cube_count_xyz(&cube_count), (4, 1, 1));
        assert_eq!(cube_dim, CubeDim::new_1d(256));
    }

    #[test]
    fn test_launch_config_256() {
        let (cube_count, _) = calculate_launch_config(256);
        // Exactly one workgroup
        assert_eq!(cube_count_xyz(&cube_count), (1, 1, 1));
    }

    #[test]
    fn test_launch_config_1() {
        let (cube_count, _) = calculate_launch_config(1);
        assert_eq!(cube_count_xyz(&cube_count), (1, 1, 1));
    }

    #[test]
    fn test_launch_config_0() {
        let (cube_count, cube_dim) = calculate_launch_config(0);
        assert_eq!(cube_count_xyz(&cube_count), (0, 1, 1));
        assert_eq!(cube_dim, CubeDim::new_1d(256));
    }

    #[test]
    fn test_launch_config_257() {
        let (cube_count, _) = calculate_launch_config(257);
        // (257 + 255) / 256 = 2
        assert_eq!(cube_count_xyz(&cube_count), (2, 1, 1));
    }

    // --- Buffer management tests ---

    #[test]
    fn test_zero_output_buffer_is_zeros() {
        let client = cpu_client();
        let n = 10;
        let handle = create_zero_output_buffer(&client, n);
        let result = read_output_buffer(&client, handle, n);
        assert_eq!(result.len(), n);
        for (i, &val) in result.iter().enumerate() {
            assert_eq!(val, 0.0, "element {i} should be zero, got {val}");
        }
    }

    #[test]
    fn test_input_buffer_roundtrip() {
        let client = cpu_client();
        let data = [1.0, -2.5, 3.14159, f64::MAX, f64::MIN_POSITIVE];
        let handle = create_input_buffer(&client, &data);
        let result = read_output_buffer(&client, handle, data.len());
        assert_eq!(result, data);
    }

    // --- Identity kernel integration tests ---

    #[test]
    fn test_identity_small() {
        let input = [1.0, 2.0, 3.0, 4.0];
        let output = run_identity(&input);
        assert_eq!(output, input);
    }

    #[test]
    fn test_identity_1000_elements() {
        let input: Vec<f64> = (0..1000).map(|i| i as f64 * 0.001).collect();
        let output = run_identity(&input);
        assert_eq!(output.len(), 1000);
        for (i, (&inp, &out)) in input.iter().zip(output.iter()).enumerate() {
            assert_eq!(
                inp.to_bits(),
                out.to_bits(),
                "bit mismatch at element {i}: input={inp}, output={out}"
            );
        }
    }

    #[test]
    fn test_identity_single_element() {
        let input = [42.0];
        let output = run_identity(&input);
        assert_eq!(output, input);
    }

    #[test]
    fn test_identity_special_values() {
        let input = [0.0, -0.0, f64::INFINITY, f64::NEG_INFINITY, f64::MIN_POSITIVE];
        let output = run_identity(&input);
        for (i, (&inp, &out)) in input.iter().zip(output.iter()).enumerate() {
            assert_eq!(
                inp.to_bits(),
                out.to_bits(),
                "bit mismatch at element {i}: input bits={:016x}, output bits={:016x}",
                inp.to_bits(),
                out.to_bits()
            );
        }
    }
}
