//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1319/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1319<F: Float>(t20904: F, t81818: F, t20953: F, t6614: F, t20978: F, t23146: F, t20861: F, t2628: F, t6605: F, t105387: F, t105390: F, t105393: F, t105396: F, t105402: F, t81921: F, t81955: F, t87387: F, t87403: F, t87405: F, t87432: F, t87445: F, t98828: F, t98830: F, t98836: F, t98838: F) -> F {
    let t105404 = t81818 * t20904;
    let t105406 = t6614 * t20953;
    let t105412 = t23146 * t20978;
    let t105415 = t6605 * t2628 * t20861;
    let t105417 = -F::new(0.25434836339308446237e-1) * t105387 + F::new(0.25434836339308446237e-1) * t105390 - F::new(0.20186378047070195427e-3) * t105393 - F::new(0.94875976821229918508e-2) * t87387 - F::new(5.0) / F::new(64.0) * t105396 + F::new(119.0) / F::new(2304.0) * t87403 - F::new(0.15812662803538319751e-2) * t87405 - F::new(35.0) / F::new(192.0) * t98828 + F::new(7.0) / F::new(96.0) * t98830 + t105402 / F::new(256.0) - t105404 / F::new(256.0) - t105406 / F::new(1536.0) - F::new(0.84782787797694820794e-2) * t98836 - F::new(0.33913115119077928317e-1) * t87432 - F::new(0.50869672678616892476e-1) * t98838 - t81921 + F::new(0.3027956707060529314e-3) * t87445 - t81955 + t105412 / F::new(128.0) + F::new(0.12111826828242117256e-2) * t105415;
    t105417
}
