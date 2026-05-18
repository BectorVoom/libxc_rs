//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 564/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk564<F: Float>(t14617: F, t656: F, t2145: F, t2069: F, t699: F, t1550: F, t2074: F, t903: F, t14105: F, t2191: F, t3219: F, t1986: F, t2229: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14618 = t14617 * t656;
    let t14619 = t2145 * t14618;
    let t14620 = F::new(0.34093327067806677161e-2) * t14619;
    let t14623 = t699 * t2069;
    let t14624 = t1550 * t14623;
    let t14625 = F::new(0.2993560425465952141e-1) * t14624;
    let t14626 = t699 * t2074;
    let t14627 = t903 * t14626;
    let t14628 = F::new(0.44903406381989282115e-1) * t14627;
    let t14630 = F::new(0.14967802127329760705e-1) * t14105;
    let t14637 = t2191 * t3219;
    let t14638 = F::new(0.42564599893297839398e-5) * t14637;
    let t14639 = t1986 * t2229;
    (t14618, t14620, t14623, t14625, t14626, t14628, t14630, t14638, t14639)
}
