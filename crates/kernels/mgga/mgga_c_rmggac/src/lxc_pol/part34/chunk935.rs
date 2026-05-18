//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 935/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk935<F: Float>(t76680: F, t7720: F, t73803: F, t73805: F, t73795: F, t73801: F, t76648: F, t76652: F, t76656: F, t76658: F, t76662: F, t76666: F, t76668: F, t76670: F, t76671: F, t76673: F, t76674: F, t76679: F) -> F {
    let t76681 = t7720 * t76680;
    let t76682 = F::new(0.12769379967989351819e-4) * t76681;
    let t76683 = F::new(0.85129199786595678799e-5) * t73803;
    let t76684 = F::new(0.85129199786595678799e-5) * t73805;
    let t76685 = -t76648 - t76652 - t76656 + t76658 + t76662 + t76666 + t76668 - t76670 + t76671 + F::new(0.87596530464506835935e-6) * t73795 - t76673 + t76674 + F::new(0.87596530464506835935e-6) * t73801 - t76679 + t76682 - t76683 - t76684;
    t76685
}
