//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2241/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2241<F: Float>(t25555: F, t82822: F, t25529: F, t6680: F, t1920: F, t2966: F, t7614: F, t14622: F, t14651: F, t1539: F, t1610: F, t23478: F, t23633: F, t23635: F, t23685: F, t23707: F, t25567: F, t25712: F, t3200: F, t4684: F, t61774: F, t6687: F, t6784: F, t6800: F, t6811: F, t7619: F, t82566: F, t82799: F, t82806: F) -> F {
    let t89421 = F::cast_from(0.18277045187202515961e-2_f64) * t82822 * t25555;
    let t89429 = F::cast_from(0.14621636149762012769e-1_f64) * t6680 * t25529;
    let t89431 = t1920 * t2966 * t7614;
    let t89433 = -F::new(2.0) * t3200 * t25567 * t4684 + t82799 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t25712 * t23478 * t23685 - t3200 * t7619 * t14622 + F::cast_from(0.26806332941230356743e-1_f64) * t82806 + F::cast_from(0.27415567780803773942e-2_f64) * t6687 * t6784 * t82566 * t1539 + t1610 * t23707 + t89421 + F::cast_from(0.54831135561607547884e-2_f64) * t23633 * t23635 * t61774 * t6800 + F::new(2.0) * t14651 * t6811 - t89429 - F::cast_from(0.18277045187202515961e-2_f64) * t89431;
    t89433
}
