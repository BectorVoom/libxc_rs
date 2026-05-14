//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 515/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk515<F: Float>(t14589: F, t2147: F, t13982: F, t13986: F, t13990: F, t13994: F, t14005: F, t14013: F, t14054: F, t14060: F, t14094: F, t22: F, t2227: F, t656: F, t2145: F, t2069: F, t699: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14590 = t14589 * t2147;
    let t14591 = 0.68186654135613354322e-2 * t14590;
    let t14592 = 0.30487649791575028312e-3 * t13982;
    let t14593 = 0.30487649791575028312e-3 * t13986;
    let t14594 = 0.20455996240684006298e-1 * t13990;
    let t14595 = 0.2727466165424534173e-1 * t13994;
    let t14596 = 0.13637330827122670865e-1 * t14005;
    let t14598 = 0.2627895913935205078e-5 * t14013;
    let t14607 = 0.19709219354514038085e-5 * t14054;
    let t14609 = 0.2627895913935205078e-5 * t14060;
    let t14616 = 0.10227998120342003148e-1 * t14094;
    let t14617 = t2227 * t22;
    let t14618 = t14617 * t656;
    let t14619 = t2145 * t14618;
    let t14620 = 0.34093327067806677161e-2 * t14619;
    let t14623 = t699 * t2069;
    (t14591, t14592, t14593, t14594, t14595, t14596, t14598, t14607, t14609, t14616, t14617, t14618, t14620, t14623)
}
