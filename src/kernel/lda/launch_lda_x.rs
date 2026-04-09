//! Safe wrapper functions for LDA_X CubeCL kernel launches.
//!
//! This module encapsulates all `unsafe { launch_unchecked }` calls for LDA_X
//! kernel functions, satisfying BUILD-04: all unsafe kernel launch code is
//! confined to `src/kernel/lda/`.
//!
//! Each wrapper function takes typed parameters (CubeCL client, launch config,
//! ArrayArg, scalar f64 values) and contains exactly one `unsafe` block wrapping
//! the `launch_unchecked` call for the corresponding kernel.

use cubecl::cpu::CpuRuntime;
use cubecl::client::ComputeClient;
use cubecl::prelude::*;

use super::lda_x;

// ============================================================================
// UNPOLARIZED WRAPPERS
// ============================================================================

/// Safe wrapper for `lda_x_exc_unpol` kernel launch.
pub fn launch_lda_x_exc_unpol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: ArrayArg<'_, CpuRuntime>,
    zk: ArrayArg<'_, CpuRuntime>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    unsafe {
        lda_x::lda_x_exc_unpol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            rho, zk,
            ScalarArg::new(alpha),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        ).unwrap();
    }
}

/// Safe wrapper for `lda_x_vxc_unpol` kernel launch.
pub fn launch_lda_x_vxc_unpol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: ArrayArg<'_, CpuRuntime>,
    zk: ArrayArg<'_, CpuRuntime>,
    vrho: ArrayArg<'_, CpuRuntime>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    unsafe {
        lda_x::lda_x_vxc_unpol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            rho, zk, vrho,
            ScalarArg::new(alpha),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        ).unwrap();
    }
}

/// Safe wrapper for `lda_x_fxc_unpol` kernel launch.
pub fn launch_lda_x_fxc_unpol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: ArrayArg<'_, CpuRuntime>,
    zk: ArrayArg<'_, CpuRuntime>,
    vrho: ArrayArg<'_, CpuRuntime>,
    v2rho2: ArrayArg<'_, CpuRuntime>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    unsafe {
        lda_x::lda_x_fxc_unpol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            rho, zk, vrho, v2rho2,
            ScalarArg::new(alpha),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        ).unwrap();
    }
}

/// Safe wrapper for `lda_x_kxc_unpol` kernel launch.
pub fn launch_lda_x_kxc_unpol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: ArrayArg<'_, CpuRuntime>,
    zk: ArrayArg<'_, CpuRuntime>,
    vrho: ArrayArg<'_, CpuRuntime>,
    v2rho2: ArrayArg<'_, CpuRuntime>,
    v3rho3: ArrayArg<'_, CpuRuntime>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    unsafe {
        lda_x::lda_x_kxc_unpol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            rho, zk, vrho, v2rho2, v3rho3,
            ScalarArg::new(alpha),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        ).unwrap();
    }
}

/// Safe wrapper for `lda_x_lxc_unpol` kernel launch.
pub fn launch_lda_x_lxc_unpol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: ArrayArg<'_, CpuRuntime>,
    zk: ArrayArg<'_, CpuRuntime>,
    vrho: ArrayArg<'_, CpuRuntime>,
    v2rho2: ArrayArg<'_, CpuRuntime>,
    v3rho3: ArrayArg<'_, CpuRuntime>,
    v4rho4: ArrayArg<'_, CpuRuntime>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    unsafe {
        lda_x::lda_x_lxc_unpol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            rho, zk, vrho, v2rho2, v3rho3, v4rho4,
            ScalarArg::new(alpha),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        ).unwrap();
    }
}

// ============================================================================
// POLARIZED WRAPPERS
// ============================================================================

/// Safe wrapper for `lda_x_exc_pol` kernel launch.
pub fn launch_lda_x_exc_pol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: ArrayArg<'_, CpuRuntime>,
    zk: ArrayArg<'_, CpuRuntime>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    unsafe {
        lda_x::lda_x_exc_pol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            rho, zk,
            ScalarArg::new(alpha),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        ).unwrap();
    }
}

/// Safe wrapper for `lda_x_vxc_pol` kernel launch.
pub fn launch_lda_x_vxc_pol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: ArrayArg<'_, CpuRuntime>,
    zk: ArrayArg<'_, CpuRuntime>,
    vrho: ArrayArg<'_, CpuRuntime>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    unsafe {
        lda_x::lda_x_vxc_pol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            rho, zk, vrho,
            ScalarArg::new(alpha),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        ).unwrap();
    }
}

/// Safe wrapper for `lda_x_fxc_pol` kernel launch.
pub fn launch_lda_x_fxc_pol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: ArrayArg<'_, CpuRuntime>,
    zk: ArrayArg<'_, CpuRuntime>,
    vrho: ArrayArg<'_, CpuRuntime>,
    v2rho2: ArrayArg<'_, CpuRuntime>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    unsafe {
        lda_x::lda_x_fxc_pol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            rho, zk, vrho, v2rho2,
            ScalarArg::new(alpha),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        ).unwrap();
    }
}

/// Safe wrapper for `lda_x_kxc_pol` kernel launch.
pub fn launch_lda_x_kxc_pol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: ArrayArg<'_, CpuRuntime>,
    zk: ArrayArg<'_, CpuRuntime>,
    vrho: ArrayArg<'_, CpuRuntime>,
    v2rho2: ArrayArg<'_, CpuRuntime>,
    v3rho3: ArrayArg<'_, CpuRuntime>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    unsafe {
        lda_x::lda_x_kxc_pol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            rho, zk, vrho, v2rho2, v3rho3,
            ScalarArg::new(alpha),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        ).unwrap();
    }
}

/// Safe wrapper for `lda_x_lxc_pol` kernel launch.
pub fn launch_lda_x_lxc_pol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: ArrayArg<'_, CpuRuntime>,
    zk: ArrayArg<'_, CpuRuntime>,
    vrho: ArrayArg<'_, CpuRuntime>,
    v2rho2: ArrayArg<'_, CpuRuntime>,
    v3rho3: ArrayArg<'_, CpuRuntime>,
    v4rho4: ArrayArg<'_, CpuRuntime>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    unsafe {
        lda_x::lda_x_lxc_pol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            rho, zk, vrho, v2rho2, v3rho3, v4rho4,
            ScalarArg::new(alpha),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        ).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::launch::{
        calculate_launch_config, cpu_client, create_input_buffer,
        create_zero_output_buffer, read_output_buffer,
    };

    /// Verify that the safe wrapper for exc_unpol produces correct results.
    /// The result should match a direct unsafe call to the kernel.
    #[test]
    fn test_launch_lda_x_exc_unpol_produces_negative_energy() {
        let client = cpu_client();
        let np = 4;
        let rho_data = [0.1, 0.2, 0.5, 1.0];
        let alpha = 1.0;
        let dens_threshold = 1e-15;
        let zeta_threshold = 1e-10;

        let rho_handle = create_input_buffer(&client, &rho_data);
        let zk_handle = create_zero_output_buffer(&client, np);
        let (cube_count, cube_dim) = calculate_launch_config(np);

        // SAFETY: handles are valid buffers created above with correct sizes.
        unsafe {
            launch_lda_x_exc_unpol(
                &client,
                cube_count,
                cube_dim,
                ArrayArg::from_raw_parts::<f64>(&rho_handle, np, 1),
                ArrayArg::from_raw_parts::<f64>(&zk_handle, np, 1),
                alpha,
                dens_threshold,
                zeta_threshold,
            );
        }

        let zk = read_output_buffer(&client, zk_handle, np);
        assert_eq!(zk.len(), np);
        // Exchange energy must be negative for all positive densities
        for (i, &val) in zk.iter().enumerate() {
            assert!(val < 0.0, "zk[{i}] = {val} should be negative");
        }
    }

    /// Verify vxc wrapper produces both zk and vrho output.
    #[test]
    fn test_launch_lda_x_vxc_unpol_produces_both_outputs() {
        let client = cpu_client();
        let np = 2;
        let rho_data = [0.1, 0.5];
        let alpha = 1.0;
        let dens_threshold = 1e-15;
        let zeta_threshold = 1e-10;

        let rho_handle = create_input_buffer(&client, &rho_data);
        let zk_handle = create_zero_output_buffer(&client, np);
        let vrho_handle = create_zero_output_buffer(&client, np);
        let (cube_count, cube_dim) = calculate_launch_config(np);

        // SAFETY: handles are valid buffers created above with correct sizes.
        unsafe {
            launch_lda_x_vxc_unpol(
                &client,
                cube_count,
                cube_dim,
                ArrayArg::from_raw_parts::<f64>(&rho_handle, np, 1),
                ArrayArg::from_raw_parts::<f64>(&zk_handle, np, 1),
                ArrayArg::from_raw_parts::<f64>(&vrho_handle, np, 1),
                alpha,
                dens_threshold,
                zeta_threshold,
            );
        }

        let zk = read_output_buffer(&client, zk_handle, np);
        let vrho = read_output_buffer(&client, vrho_handle, np);

        for (i, &val) in zk.iter().enumerate() {
            assert!(val < 0.0, "zk[{i}] = {val} should be negative");
        }
        for (i, &val) in vrho.iter().enumerate() {
            assert!(val != 0.0, "vrho[{i}] should be non-zero");
        }
    }

    /// Verify polarized exc wrapper works correctly.
    #[test]
    fn test_launch_lda_x_exc_pol_produces_negative_energy() {
        let client = cpu_client();
        let np = 2;
        // Polarized: 2 components per point [rho_a, rho_b, rho_a, rho_b]
        let rho_data = [0.1, 0.05, 0.2, 0.1];
        let alpha = 1.0;
        let dens_threshold = 1e-15;
        let zeta_threshold = 1e-10;

        let rho_handle = create_input_buffer(&client, &rho_data);
        let zk_handle = create_zero_output_buffer(&client, np);
        let (cube_count, cube_dim) = calculate_launch_config(np);

        // SAFETY: handles are valid buffers created above with correct sizes.
        unsafe {
            launch_lda_x_exc_pol(
                &client,
                cube_count,
                cube_dim,
                ArrayArg::from_raw_parts::<f64>(&rho_handle, np * 2, 1),
                ArrayArg::from_raw_parts::<f64>(&zk_handle, np, 1),
                alpha,
                dens_threshold,
                zeta_threshold,
            );
        }

        let zk = read_output_buffer(&client, zk_handle, np);
        for (i, &val) in zk.iter().enumerate() {
            assert!(val < 0.0, "zk[{i}] = {val} should be negative");
        }
    }
}
