//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2622/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2622<F: Float>(t11786: F, t5024: F, t3509: F, t607: F, t3032: F, t52434: F, t3505: F, t1090: F, t11678: F, t1174: F, t11855: F, t1196: F, t15525: F, t15591: F, t3252: F, t3496: F, t3511: F, t3577: F, t3578: F, t45222: F, t45224: F, t45227: F, t45872: F, t4728: F, t5002: F, t5012: F, t974: F) -> (F, F, F) {
    let t53360 = t5024 * t11786;
    let t53366 = t3509 * t607;
    let t53371 = t52434 * t3032;
    let t53372 = t53371 * t3505;
    let t53377 = t5002 * t11855 / F::cast_from(3072.0_f64) - t1174 * t974 * t1196 * t45872 / F::cast_from(288.0_f64) - t3577 * t3578 * t15525 * t1090 / F::cast_from(1536.0_f64) - t45222 / F::cast_from(144.0_f64) - t45224 / F::cast_from(4608.0_f64) + t45227 / F::cast_from(216.0_f64) - F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t53360 - t3577 * t3578 * t5012 * t3252 / F::cast_from(1536.0_f64) - t11678 * t3578 * t4728 * t53366 / F::cast_from(384.0_f64) + t53372 * t3511 / F::cast_from(512.0_f64) + t15591 * t3496 / F::cast_from(1024.0_f64);
    (t53366, t53371, t53377)
}
