//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 571/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk571<F: Float>(t14043: F, t14049: F, t14054: F, t14057: F, t14060: F, t3230: F, t504: F, t14094: F, t22: F, t2227: F, t656: F, t2145: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14602 = F::new(0.4379826523225341797e-6) * t14043;
    let t14603 = F::new(0.87596530464506835935e-6) * t14049;
    let t14607 = F::new(0.19709219354514038085e-5) * t14054;
    let t14608 = F::new(0.87596530464506835935e-6) * t14057;
    let t14609 = F::new(0.2627895913935205078e-5) * t14060;
    let t14611 = t504 * t3230;
    let t14612 = F::new(0.19957069503106347607e-1) * t14611;
    let t14616 = F::new(0.10227998120342003148e-1) * t14094;
    let t14617 = t2227 * t22;
    let t14618 = t14617 * t656;
    let t14619 = t2145 * t14618;
    (t14602, t14603, t14607, t14608, t14609, t14612, t14616, t14617, t14618, t14619)
}
