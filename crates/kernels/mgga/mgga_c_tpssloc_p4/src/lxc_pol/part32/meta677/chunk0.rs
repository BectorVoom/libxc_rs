//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2114/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2114<F: Float>(t24740: F, t5064: F, t15640: F, t24729: F, t24574: F, t27574: F, t24844: F, t7999: F, t2121: F, t3427: F, t8077: F, t27517: F, t85639: F) -> (F, F, F, F, F, F) {
    let t95687 = t5064 * t24740;
    let t95702 = t24729 * t15640 / F::new(576.0);
    let t95714 = F::cast_from(0.54831135561607547884e-2_f64) * t24574 * t27574;
    let t95722 = F::cast_from(0.14621636149762012769e-1_f64) * t7999 * t24844;
    let t95726 = t2121 * t3427 * t8077;
    let t95747 = F::cast_from(0.18277045187202515961e-2_f64) * t85639 * t27517;
    (t95687, t95702, t95714, t95722, t95726, t95747)
}
