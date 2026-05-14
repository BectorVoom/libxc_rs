//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 518/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk518<F: Float>(t14060: F, t3230: F, t504: F, t14094: F, t22: F, t2227: F, t656: F, t2145: F, t14501: F, t739: F, t2069: F, t699: F, t1550: F, t2074: F, t903: F, t14105: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14609 = 0.2627895913935205078e-5 * t14060;
    let t14611 = t504 * t3230;
    let t14612 = 0.19957069503106347607e-1 * t14611;
    let t14616 = 0.10227998120342003148e-1 * t14094;
    let t14617 = t2227 * t22;
    let t14618 = t14617 * t656;
    let t14619 = t2145 * t14618;
    let t14620 = 0.34093327067806677161e-2 * t14619;
    let t14621 = t739 * t14501;
    let t14622 = 0.59871208509319042821e-1 * t14621;
    let t14623 = t699 * t2069;
    let t14624 = t1550 * t14623;
    let t14625 = 0.2993560425465952141e-1 * t14624;
    let t14626 = t699 * t2074;
    let t14627 = t903 * t14626;
    let t14628 = 0.44903406381989282115e-1 * t14627;
    let t14630 = 0.14967802127329760705e-1 * t14105;
    (t14609, t14612, t14616, t14617, t14618, t14620, t14622, t14623, t14625, t14626, t14628, t14630)
}
