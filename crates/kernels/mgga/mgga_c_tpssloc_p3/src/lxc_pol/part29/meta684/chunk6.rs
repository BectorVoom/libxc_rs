//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2332/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2332<F: Float>(t24585: F, t7999: F, t24574: F, t27800: F, t225: F, t27805: F, t11613: F, t1191: F, t1238: F, t1241: F, t1252: F, t15802: F, t1720: F, t2155: F, t24612: F, t24757: F, t24897: F, t254: F, t27784: F, t27785: F, t27786: F, t27792: F, t3631: F, t4940: F, t498: F, t5055: F, t53703: F, t7348: F, t8088: F, t94779: F, t94820: F, t94867: F, t94902: F, t94942: F, t94980: F, t95026: F, t95058: F, t95087: F, t95122: F, t95150: F, t95184: F, t95224: F, t95723: F, t95752: F, t95779: F, t95817: F) -> F {
    let t95824 = t7999 * t24585;
    let t95834 = F::cast_from(0.54831135561607547884e-2_f64) * t24574 * t27800;
    let t95836 = t27805 * t225;
    let t95844 = -F::new(2.0) * t53703 * t2155 - F::new(6.0) * t27784 * t27785 * t15802 - F::new(2.0) * t11613 * t8088 - t94779 - t1238 * t1241 * (t94820 + t94867 + t94902 + t94942 + t94980 + t95026 + t95058 + t95087 + t95122 + t95150 + t95184 + t95224 + t95723 + t95752 + t95779 + t95817) + F::cast_from(0.48738787165873375895e-2_f64) * t95824 - F::cast_from(0.21932454224643019153e-1_f64) * t7999 * t24612 - F::new(12.0) * t1191 * t254 * t27786 - F::new(6.0) * t5055 * t24897 - t95834 - t27792 * t3631 - F::new(2.0) * t95836 * t1252 + F::new(2.0) * t4940 * t7348 * t498 + t1720 * t24757 * t498;
    t95844
}
