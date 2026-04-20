//! Match-based dispatch layer for LDA kernel evaluation.
//!
//! Routes evaluation requests to the correct CubeCL kernel function based on
//! derivative order and spin mode. All kernel launches go through the safe
//! wrappers in `kernel::lda::launch_lda_x` -- this module contains no raw kernel
//! launch calls (BUILD-04 compliant).

use crate::dims::Dimensions;
use crate::error::LibxcRsError;
use crate::input::LdaInput;
use crate::kernel::launch::{
    calculate_launch_config, cpu_client, create_input_buffer,
    create_zero_output_buffer, read_output_buffer,
};
use crate::kernel::lda::lda_x::{exc_unpol, exc_pol, fxc_unpol, fxc_pol, kxc_unpol, kxc_pol, lxc_unpol, lxc_pol, vxc_unpol, vxc_pol};
use cubecl::cpu::CpuRuntime;
use cubecl::frontend::ScalarArg;
use cubecl::prelude::{ArrayArg, LaunchError};
use crate::model::{DerivativeOrder, Spin, Thresholds};
use crate::output::LdaOutput;

/// Evaluate an LDA functional on the given input, writing results to output.
///
/// Routes to the correct kernel based on derivative order and spin mode.
/// Zeros caller output buffers before evaluation. Handles `None` output
/// fields by allocating dummy buffers the kernel writes to but whose
/// results are discarded.
///
/// **Note on `zk`:** The energy density `zk` is always computed by every LDA
/// kernel variant, regardless of the requested derivative order. If
/// `output.zk` is `None`, a dummy GPU buffer is still allocated and written
/// by the kernel, but the result is not copied back to the caller. Passing
/// `None` for `zk` therefore wastes one output buffer allocation but does
/// not skip computation. Higher-order derivative fields (`vrho`, `v2rho2`,
/// etc.) are truly optional and only computed when `order` requires them.
///
/// # Arguments
/// * `input` - Validated LDA input bundle
/// * `order` - Maximum derivative order to compute
/// * `output` - Output bundle with optional buffers for each derivative level
/// * `alpha` - Functional mixing parameter (1.0 for pure LDA_X)
/// * `thresholds` - Numerical thresholds for evaluation stability
///
/// # Errors
/// Returns `LibxcRsError` if evaluation fails.
pub fn dispatch_lda(
    input: &LdaInput,
    order: DerivativeOrder,
    output: &mut LdaOutput,
    alpha: f64,
    thresholds: &Thresholds,
) -> Result<(), LibxcRsError> {
    let np = input.np();
    let spin = input.spin();
    let dims = Dimensions::lda(spin);

    // Zero caller-provided output buffers (T-03-04 mitigation).
    // Kernels use += accumulation, so stale data would corrupt results.
    //
    // INVARIANT: dispatch_lda is authoritative for zeroing its own output
    // buffers. Callers (including evaluate_mixed_lda) must NOT rely on
    // this zeroing for their own accumulation logic -- they should zero
    // their own output separately. The mixed path zeros scratch via
    // workspace.zero_scratch() before building scratch_output, and
    // dispatch_lda re-zeros those same slices here. The double-zero is
    // intentional: it keeps dispatch_lda self-contained so it can be
    // called directly (not only through the mixed path).
    if let Some(ref mut buf) = output.zk {
        buf.fill(0.0);
    }
    if let Some(ref mut buf) = output.vrho {
        buf.fill(0.0);
    }
    if let Some(ref mut buf) = output.v2rho2 {
        buf.fill(0.0);
    }
    if let Some(ref mut buf) = output.v3rho3 {
        buf.fill(0.0);
    }
    if let Some(ref mut buf) = output.v4rho4 {
        buf.fill(0.0);
    }

    // Create CubeCL client and upload input
    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, input.rho());
    let rho_len = input.rho().len();

    // Create output handles for each derivative level up to `order`.
    // For None output fields, create dummy buffers the kernel writes to
    // but whose results are discarded (D-07 bridge).
    let zk_len = np * dims.zk as usize;
    let zk_handle = create_zero_output_buffer(&client, zk_len);

    let vrho_len = np * dims.vrho as usize;
    let vrho_handle = if order >= DerivativeOrder::Vxc {
        Some(create_zero_output_buffer(&client, vrho_len))
    } else {
        None
    };

    let v2rho2_len = np * dims.v2rho2 as usize;
    let v2rho2_handle = if order >= DerivativeOrder::Fxc {
        Some(create_zero_output_buffer(&client, v2rho2_len))
    } else {
        None
    };

    let v3rho3_len = np * dims.v3rho3 as usize;
    let v3rho3_handle = if order >= DerivativeOrder::Kxc {
        Some(create_zero_output_buffer(&client, v3rho3_len))
    } else {
        None
    };

    let v4rho4_len = np * dims.v4rho4 as usize;
    let v4rho4_handle = if order >= DerivativeOrder::Lxc {
        Some(create_zero_output_buffer(&client, v4rho4_len))
    } else {
        None
    };

    let (cube_count, cube_dim) = calculate_launch_config(np);

    // Dispatch to correct kernel based on (order, spin).
    // Uses direct kernel launch through CubeCL.

    // Helper to convert kernel launch errors into LibxcRsError.
    let map_launch_err = |e: LaunchError| LibxcRsError::KernelLaunchFailed {
        reason: e.to_string(),
    };
    let alpha_arg = ScalarArg { elem: alpha };
    let density_threshold_arg = ScalarArg { elem: thresholds.density };
    let zeta_threshold_arg = ScalarArg { elem: thresholds.zeta };

    match (order, spin) {
        (DerivativeOrder::Exc, Spin::Unpolarized) => {
            unsafe {
                exc_unpol::lda_x_exc_unpol::launch_unchecked::<CpuRuntime>(
                    &client, cube_count, cube_dim,
                    ArrayArg::from_raw_parts::<f64>(&rho_handle, rho_len, 1),
                    ArrayArg::from_raw_parts::<f64>(&zk_handle, zk_len, 1),
                    alpha_arg, density_threshold_arg, zeta_threshold_arg,
                ).map_err(map_launch_err)?;
            }
        }
        (DerivativeOrder::Vxc, Spin::Unpolarized) => {
            let vrho_h = vrho_handle.as_ref().unwrap();
            unsafe {
                vxc_unpol::lda_x_vxc_unpol::launch_unchecked::<CpuRuntime>(
                    &client, cube_count, cube_dim,
                    ArrayArg::from_raw_parts::<f64>(&rho_handle, rho_len, 1),
                    ArrayArg::from_raw_parts::<f64>(&zk_handle, zk_len, 1),
                    ArrayArg::from_raw_parts::<f64>(vrho_h, vrho_len, 1),
                    alpha_arg, density_threshold_arg, zeta_threshold_arg,
                ).map_err(map_launch_err)?;
            }
        }
        (DerivativeOrder::Fxc, Spin::Unpolarized) => {
            let vrho_h = vrho_handle.as_ref().unwrap();
            let v2rho2_h = v2rho2_handle.as_ref().unwrap();
            unsafe {
                fxc_unpol::lda_x_fxc_unpol::launch_unchecked::<CpuRuntime>(
                    &client, cube_count, cube_dim,
                    ArrayArg::from_raw_parts::<f64>(&rho_handle, rho_len, 1),
                    ArrayArg::from_raw_parts::<f64>(&zk_handle, zk_len, 1),
                    ArrayArg::from_raw_parts::<f64>(vrho_h, vrho_len, 1),
                    ArrayArg::from_raw_parts::<f64>(v2rho2_h, v2rho2_len, 1),
                    alpha_arg, density_threshold_arg, zeta_threshold_arg,
                ).map_err(map_launch_err)?;
            }
        }
        (DerivativeOrder::Kxc, Spin::Unpolarized) => {
            let vrho_h = vrho_handle.as_ref().unwrap();
            let v2rho2_h = v2rho2_handle.as_ref().unwrap();
            let v3rho3_h = v3rho3_handle.as_ref().unwrap();
            unsafe {
                kxc_unpol::lda_x_kxc_unpol::launch_unchecked::<CpuRuntime>(
                    &client, cube_count, cube_dim,
                    ArrayArg::from_raw_parts::<f64>(&rho_handle, rho_len, 1),
                    ArrayArg::from_raw_parts::<f64>(&zk_handle, zk_len, 1),
                    ArrayArg::from_raw_parts::<f64>(vrho_h, vrho_len, 1),
                    ArrayArg::from_raw_parts::<f64>(v2rho2_h, v2rho2_len, 1),
                    ArrayArg::from_raw_parts::<f64>(v3rho3_h, v3rho3_len, 1),
                    alpha_arg, density_threshold_arg, zeta_threshold_arg,
                ).map_err(map_launch_err)?;
            }
        }
        (DerivativeOrder::Lxc, Spin::Unpolarized) => {
            let vrho_h = vrho_handle.as_ref().unwrap();
            let v2rho2_h = v2rho2_handle.as_ref().unwrap();
            let v3rho3_h = v3rho3_handle.as_ref().unwrap();
            let v4rho4_h = v4rho4_handle.as_ref().unwrap();
            unsafe {
                lxc_unpol::lda_x_lxc_unpol::launch_unchecked::<CpuRuntime>(
                    &client, cube_count, cube_dim,
                    ArrayArg::from_raw_parts::<f64>(&rho_handle, rho_len, 1),
                    ArrayArg::from_raw_parts::<f64>(&zk_handle, zk_len, 1),
                    ArrayArg::from_raw_parts::<f64>(vrho_h, vrho_len, 1),
                    ArrayArg::from_raw_parts::<f64>(v2rho2_h, v2rho2_len, 1),
                    ArrayArg::from_raw_parts::<f64>(v3rho3_h, v3rho3_len, 1),
                    ArrayArg::from_raw_parts::<f64>(v4rho4_h, v4rho4_len, 1),
                    alpha_arg, density_threshold_arg, zeta_threshold_arg,
                ).map_err(map_launch_err)?;
            }
        }
        (DerivativeOrder::Exc, Spin::Polarized) => {
            unsafe {
                exc_pol::lda_x_exc_pol::launch_unchecked::<CpuRuntime>(
                    &client, cube_count, cube_dim,
                    ArrayArg::from_raw_parts::<f64>(&rho_handle, rho_len, 1),
                    ArrayArg::from_raw_parts::<f64>(&zk_handle, zk_len, 1),
                    alpha_arg, density_threshold_arg, zeta_threshold_arg,
                ).map_err(map_launch_err)?;
            }
        }
        (DerivativeOrder::Vxc, Spin::Polarized) => {
            let vrho_h = vrho_handle.as_ref().unwrap();
            unsafe {
                vxc_pol::lda_x_vxc_pol::launch_unchecked::<CpuRuntime>(
                    &client, cube_count, cube_dim,
                    ArrayArg::from_raw_parts::<f64>(&rho_handle, rho_len, 1),
                    ArrayArg::from_raw_parts::<f64>(&zk_handle, zk_len, 1),
                    ArrayArg::from_raw_parts::<f64>(vrho_h, vrho_len, 1),
                    alpha_arg, density_threshold_arg, zeta_threshold_arg,
                ).map_err(map_launch_err)?;
            }
        }
        (DerivativeOrder::Fxc, Spin::Polarized) => {
            let vrho_h = vrho_handle.as_ref().unwrap();
            let v2rho2_h = v2rho2_handle.as_ref().unwrap();
            unsafe {
                fxc_pol::lda_x_fxc_pol::launch_unchecked::<CpuRuntime>(
                    &client, cube_count, cube_dim,
                    ArrayArg::from_raw_parts::<f64>(&rho_handle, rho_len, 1),
                    ArrayArg::from_raw_parts::<f64>(&zk_handle, zk_len, 1),
                    ArrayArg::from_raw_parts::<f64>(vrho_h, vrho_len, 1),
                    ArrayArg::from_raw_parts::<f64>(v2rho2_h, v2rho2_len, 1),
                    alpha_arg, density_threshold_arg, zeta_threshold_arg,
                ).map_err(map_launch_err)?;
            }
        }
        (DerivativeOrder::Kxc, Spin::Polarized) => {
            let vrho_h = vrho_handle.as_ref().unwrap();
            let v2rho2_h = v2rho2_handle.as_ref().unwrap();
            let v3rho3_h = v3rho3_handle.as_ref().unwrap();
            unsafe {
                kxc_pol::lda_x_kxc_pol::launch_unchecked::<CpuRuntime>(
                    &client, cube_count, cube_dim,
                    ArrayArg::from_raw_parts::<f64>(&rho_handle, rho_len, 1),
                    ArrayArg::from_raw_parts::<f64>(&zk_handle, zk_len, 1),
                    ArrayArg::from_raw_parts::<f64>(vrho_h, vrho_len, 1),
                    ArrayArg::from_raw_parts::<f64>(v2rho2_h, v2rho2_len, 1),
                    ArrayArg::from_raw_parts::<f64>(v3rho3_h, v3rho3_len, 1),
                    alpha_arg, density_threshold_arg, zeta_threshold_arg,
                ).map_err(map_launch_err)?;
            }
        }
        (DerivativeOrder::Lxc, Spin::Polarized) => {
            let vrho_h = vrho_handle.as_ref().unwrap();
            let v2rho2_h = v2rho2_handle.as_ref().unwrap();
            let v3rho3_h = v3rho3_handle.as_ref().unwrap();
            let v4rho4_h = v4rho4_handle.as_ref().unwrap();
            unsafe {
                lxc_pol::lda_x_lxc_pol::launch_unchecked::<CpuRuntime>(
                    &client, cube_count, cube_dim,
                    ArrayArg::from_raw_parts::<f64>(&rho_handle, rho_len, 1),
                    ArrayArg::from_raw_parts::<f64>(&zk_handle, zk_len, 1),
                    ArrayArg::from_raw_parts::<f64>(vrho_h, vrho_len, 1),
                    ArrayArg::from_raw_parts::<f64>(v2rho2_h, v2rho2_len, 1),
                    ArrayArg::from_raw_parts::<f64>(v3rho3_h, v3rho3_len, 1),
                    ArrayArg::from_raw_parts::<f64>(v4rho4_h, v4rho4_len, 1),
                    alpha_arg, density_threshold_arg, zeta_threshold_arg,
                ).map_err(map_launch_err)?;
            }
        }
    }

    // Read back results from CubeCL buffers into caller-provided output slices.
    // Only copy back for Some fields; None fields had dummy buffers that are discarded.
    // Each readback includes a length check to return a typed error instead of
    // panicking via copy_from_slice on size mismatch.
    if let Some(ref mut buf) = output.zk {
        let result = read_output_buffer(&client, zk_handle, zk_len);
        if buf.len() != result.len() {
            return Err(LibxcRsError::OutputBufferSizeMismatch {
                field: "zk",
                expected: buf.len(),
                actual: result.len(),
            });
        }
        buf.copy_from_slice(&result);
    }
    if let (Some(buf), Some(h)) = (&mut output.vrho, vrho_handle) {
        let result = read_output_buffer(&client, h, vrho_len);
        if buf.len() != result.len() {
            return Err(LibxcRsError::OutputBufferSizeMismatch {
                field: "vrho",
                expected: buf.len(),
                actual: result.len(),
            });
        }
        buf.copy_from_slice(&result);
    }
    if let (Some(buf), Some(h)) = (&mut output.v2rho2, v2rho2_handle) {
        let result = read_output_buffer(&client, h, v2rho2_len);
        if buf.len() != result.len() {
            return Err(LibxcRsError::OutputBufferSizeMismatch {
                field: "v2rho2",
                expected: buf.len(),
                actual: result.len(),
            });
        }
        buf.copy_from_slice(&result);
    }
    if let (Some(buf), Some(h)) = (&mut output.v3rho3, v3rho3_handle) {
        let result = read_output_buffer(&client, h, v3rho3_len);
        if buf.len() != result.len() {
            return Err(LibxcRsError::OutputBufferSizeMismatch {
                field: "v3rho3",
                expected: buf.len(),
                actual: result.len(),
            });
        }
        buf.copy_from_slice(&result);
    }
    if let (Some(buf), Some(h)) = (&mut output.v4rho4, v4rho4_handle) {
        let result = read_output_buffer(&client, h, v4rho4_len);
        if buf.len() != result.len() {
            return Err(LibxcRsError::OutputBufferSizeMismatch {
                field: "v4rho4",
                expected: buf.len(),
                actual: result.len(),
            });
        }
        buf.copy_from_slice(&result);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::LdaInput;
    use crate::model::{DerivativeOrder, Spin, Thresholds};
    use crate::output::LdaOutput;

    fn default_thresholds() -> Thresholds {
        Thresholds::default()
    }

    #[test]
    fn test_exc_unpolarized_produces_negative_energy() {
        let rho = vec![0.1, 0.2, 0.5, 1.0];
        let np = 4;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), None, None, None, None, np, Spin::Unpolarized,
        ).unwrap();

        dispatch_lda(&input, DerivativeOrder::Exc, &mut output, 1.0, &default_thresholds()).unwrap();

        let zk_result = output.zk.unwrap();
        for (i, &val) in zk_result.iter().enumerate() {
            assert!(val < 0.0, "zk[{i}] = {val} should be negative");
        }
    }

    #[test]
    fn test_vxc_unpolarized_populates_both_zk_and_vrho() {
        let rho = vec![0.1, 0.5];
        let np = 2;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut vrho = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), Some(&mut vrho), None, None, None, np, Spin::Unpolarized,
        ).unwrap();

        dispatch_lda(&input, DerivativeOrder::Vxc, &mut output, 1.0, &default_thresholds()).unwrap();

        let zk_result = output.zk.unwrap();
        for (i, &val) in zk_result.iter().enumerate() {
            assert!(val < 0.0, "zk[{i}] = {val} should be negative");
        }
        let vrho_result = output.vrho.unwrap();
        for (i, &val) in vrho_result.iter().enumerate() {
            assert!(val != 0.0, "vrho[{i}] should be non-zero");
        }
    }

    #[test]
    fn test_vxc_with_vrho_none_still_succeeds() {
        let rho = vec![0.1, 0.5];
        let np = 2;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), None, None, None, None, np, Spin::Unpolarized,
        ).unwrap();

        // vrho is None but order is Vxc -- should succeed with dummy buffer
        dispatch_lda(&input, DerivativeOrder::Vxc, &mut output, 1.0, &default_thresholds()).unwrap();

        let zk_result = output.zk.unwrap();
        for (i, &val) in zk_result.iter().enumerate() {
            assert!(val < 0.0, "zk[{i}] = {val} should be negative");
        }
    }

    #[test]
    fn test_exc_polarized_routes_to_pol_kernel() {
        let np = 2;
        // Polarized: 2 components per point
        let rho = vec![0.1, 0.05, 0.2, 0.1];
        let input = LdaInput::new(&rho, np, Spin::Polarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), None, None, None, None, np, Spin::Polarized,
        ).unwrap();

        dispatch_lda(&input, DerivativeOrder::Exc, &mut output, 1.0, &default_thresholds()).unwrap();

        let zk_result = output.zk.unwrap();
        for (i, &val) in zk_result.iter().enumerate() {
            assert!(val < 0.0, "zk[{i}] = {val} should be negative (polarized)");
        }
    }

    #[test]
    fn test_vxc_polarized_vrho_has_2np_elements() {
        let np = 2;
        let rho = vec![0.1, 0.05, 0.2, 0.1];
        let input = LdaInput::new(&rho, np, Spin::Polarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut vrho = vec![0.0f64; np * 2]; // Polarized: 2 components per point
        let mut output = LdaOutput::new(
            Some(&mut zk), Some(&mut vrho), None, None, None, np, Spin::Polarized,
        ).unwrap();

        dispatch_lda(&input, DerivativeOrder::Vxc, &mut output, 1.0, &default_thresholds()).unwrap();

        let vrho_result = output.vrho.unwrap();
        assert_eq!(vrho_result.len(), np * 2);
        for (i, &val) in vrho_result.iter().enumerate() {
            assert!(val != 0.0, "vrho[{i}] should be non-zero (polarized)");
        }
    }

    #[test]
    fn test_dispatch_zeros_output_buffers() {
        let rho = vec![0.1];
        let np = 1;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![999.0f64; np]; // Pre-filled with non-zero
        let mut output = LdaOutput::new(
            Some(&mut zk), None, None, None, None, np, Spin::Unpolarized,
        ).unwrap();

        dispatch_lda(&input, DerivativeOrder::Exc, &mut output, 1.0, &default_thresholds()).unwrap();

        // The result should NOT be 999.0 + kernel_value; it should be just kernel_value
        // because dispatch zeros the buffer before launch
        let zk_result = output.zk.unwrap();
        assert!(zk_result[0] < 0.0, "zk should be negative, not contaminated by pre-fill");
        // Exchange energy for rho=0.1 is approximately -0.34 (LDA_X Slater exchange)
        assert!(zk_result[0] > -2.0, "zk = {} seems too negative (pre-fill contamination?)", zk_result[0]);
    }

    #[test]
    fn test_fxc_unpolarized_populates_all_levels() {
        let rho = vec![0.5];
        let np = 1;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut vrho = vec![0.0f64; np];
        let mut v2rho2 = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), Some(&mut vrho), Some(&mut v2rho2), None, None, np, Spin::Unpolarized,
        ).unwrap();

        dispatch_lda(&input, DerivativeOrder::Fxc, &mut output, 1.0, &default_thresholds()).unwrap();

        assert!(output.zk.unwrap()[0] < 0.0, "zk should be negative");
        assert!(output.vrho.unwrap()[0] != 0.0, "vrho should be non-zero");
        assert!(output.v2rho2.unwrap()[0] != 0.0, "v2rho2 should be non-zero");
    }

    #[test]
    fn test_kxc_unpolarized_populates_all_levels() {
        let rho = vec![0.5];
        let np = 1;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut vrho = vec![0.0f64; np];
        let mut v2rho2 = vec![0.0f64; np];
        let mut v3rho3 = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), Some(&mut vrho), Some(&mut v2rho2), Some(&mut v3rho3), None, np, Spin::Unpolarized,
        ).unwrap();

        dispatch_lda(&input, DerivativeOrder::Kxc, &mut output, 1.0, &default_thresholds()).unwrap();

        assert!(output.zk.unwrap()[0] < 0.0, "zk should be negative");
        assert!(output.vrho.unwrap()[0] != 0.0, "vrho should be non-zero");
        assert!(output.v2rho2.unwrap()[0] != 0.0, "v2rho2 should be non-zero");
        assert!(output.v3rho3.unwrap()[0] != 0.0, "v3rho3 should be non-zero");
    }

    #[test]
    fn test_lxc_unpolarized_populates_all_levels() {
        let rho = vec![0.5];
        let np = 1;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut vrho = vec![0.0f64; np];
        let mut v2rho2 = vec![0.0f64; np];
        let mut v3rho3 = vec![0.0f64; np];
        let mut v4rho4 = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), Some(&mut vrho), Some(&mut v2rho2), Some(&mut v3rho3), Some(&mut v4rho4), np, Spin::Unpolarized,
        ).unwrap();

        dispatch_lda(&input, DerivativeOrder::Lxc, &mut output, 1.0, &default_thresholds()).unwrap();

        assert!(output.zk.unwrap()[0] < 0.0, "zk should be negative");
        assert!(output.vrho.unwrap()[0] != 0.0, "vrho should be non-zero");
        assert!(output.v2rho2.unwrap()[0] != 0.0, "v2rho2 should be non-zero");
        assert!(output.v3rho3.unwrap()[0] != 0.0, "v3rho3 should be non-zero");
        assert!(output.v4rho4.unwrap()[0] != 0.0, "v4rho4 should be non-zero");
    }
}
