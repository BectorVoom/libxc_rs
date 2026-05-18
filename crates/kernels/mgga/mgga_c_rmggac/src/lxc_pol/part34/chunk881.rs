//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 881/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk881<F: Float>(t15078: F, t5016: F, t1502: F, t27: F, t16129: F, t69609: F, t34975: F, t34976: F, t665: F, t9145: F, t1326: F, t75307: F) -> (F, F, F, F) {
    let t75758 = t5016 * t15078;
    let t75760 = t27 * t1502;
    let t75762 = t69609 * t16129 * t75760;
    let t75767 = F::new(0.1064114997332445985e-4) * t34975 * t34976 * t665 * t9145;
    let t75770 = t1326 * t75307;
    (t75758, t75762, t75767, t75770)
}
