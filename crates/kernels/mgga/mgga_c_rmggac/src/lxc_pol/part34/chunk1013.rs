//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1013/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1013<F: Float>(t75533: F, t75536: F, t75561: F, t75564: F, t75566: F, t15442: F, t4041: F, t69835: F, t69837: F, t71513: F, t71514: F, t75531: F, t75539: F, t75545: F, t75550: F, t75553: F, t75556: F, t75558: F) -> F {
    let t77653 = F::cast_from(0.2627895913935205078e-5_f64) * t75533;
    let t77654 = F::cast_from(0.7883687741805615234e-5_f64) * t75536;
    let t77658 = F::cast_from(0.10511583655740820312e-4_f64) * t75561;
    let t77659 = F::cast_from(0.2627895913935205078e-5_f64) * t75564;
    let t77660 = F::cast_from(0.2627895913935205078e-5_f64) * t75566;
    let t77661 = F::cast_from(0.59871208509319042821e-1_f64) * t4041 * t15442 - t69835 - F::cast_from(0.20439190441718261719e-5_f64) * t69837 - t75531 - t77653 + t77654 + t75539 + t75545 + t75550 - F::cast_from(0.35038612185802734376e-6_f64) * t75553 + F::cast_from(0.35038612185802734376e-6_f64) * t75556 - F::cast_from(0.8725742978126057077e-4_f64) * t75558 - t77658 - t77659 + t77660 - t71513 - t71514;
    t77661
}
