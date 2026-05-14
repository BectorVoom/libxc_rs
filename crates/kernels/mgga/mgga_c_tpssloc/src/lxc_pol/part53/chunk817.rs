//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 817/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk817<F: Float>(t22986: F, t33448: F, t1527: F, t31332: F, t23270: F, t1888: F, t32212: F, t33159: F, t5161: F, t8804: F, t1842: F, t8800: F, t3887: F, t2091: F, t7936: F, t12021: F, t8793: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33449 = t22986 * t33448;
    let t33457 = t31332 * t1527;
    let t33458 = t23270 * t33457;
    let t33459 = t1888 * t33458;
    let t33790 = t32212 * t33159;
    let t33793 = t8804 * t5161;
    let t33797 = t8800 * t1842;
    let t33798 = t3887 * t33797;
    let t33804 = t3887 * t2091 * t7936;
    let t33810 = t12021 * t8793 * t1842;
    (t33449, t33457, t33458, t33459, t33790, t33793, t33798, t33804, t33810)
}
