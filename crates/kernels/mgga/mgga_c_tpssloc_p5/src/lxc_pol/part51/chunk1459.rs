//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1459/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1459<F: Float>(t31304: F, t7754: F, t33366: F, t6876: F, t24994: F, t8606: F, t24996: F, t122678: F, t122681: F, t122685: F, t122692: F, t1266: F, t1459: F, t1869: F, t2036: F, t25958: F, t26870: F, t31246: F, t31532: F, t33133: F, t33579: F, t4037: F, t7040: F, t7171: F, t7670: F, t7943: F) -> F {
    let t122696 = t31304 * t7754;
    let t122697 = t6876 * t33366;
    let t122698 = t8606 * t24994;
    let t122700 = F::cast_from(6.0_f64) * t122698 * t24996;
    let t122701 = -F::cast_from(2.0_f64) * t122685 * t1459 - t1266 * t33579 - t1869 * t26870 - t2036 * t25958 - t31246 * t7943 - F::cast_from(2.0_f64) * t31532 * t4037 + F::cast_from(3.0_f64) * t33133 * t7171 - t7040 * t7670 + t122678 - t122681 - t122692 + t122696 - t122697 + t122700;
    t122701
}
