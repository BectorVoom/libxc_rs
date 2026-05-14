//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1083/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1083<F: Float>(t32781: F, t532: F, t1983: F, t6879: F, t33160: F, t6876: F, t26502: F, t3701: F, t2019: F, t24990: F, t31047: F, t25994: F, t8526: F, t26114: F, t8327: F, t33211: F, t6535: F) -> (F, F, F, F, F, F, F) {
    let t119999 = t532 * t32781;
    let t120002 = 3.0 * t1983 * t119999 * t6879;
    let t120008 = 3.0 * t6876 * t33160;
    let t120016 = t3701 * t26502;
    let t120019 = 2.0 * t1983 * t2019 * t120016;
    let t120044 = 3.0 * t1983 * t31047 * t24990;
    let t120063 = 4.0 * t8526 * t25994;
    let t120067 = 2.0 * t26114 * t8327;
    let t120069 = 4.0 * t33211 * t6535;
    (t120002, t120008, t120019, t120044, t120063, t120067, t120069)
}
