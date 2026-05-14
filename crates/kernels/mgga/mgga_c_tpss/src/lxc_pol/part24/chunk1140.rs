//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1140/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1140<F: Float>(t2677: F, t5610: F, t2680: F, t5608: F, t938: F, t5599: F, t762: F, t2689: F, t5605: F, t5614: F, t962: F, t1723: F, t2650: F, t219: F, t5624: F, t5570: F, t5628: F) -> (F, F, F, F, F, F, F, F) {
    let t18104 = t5610 * t2677;
    let t18107 = t938 * t5608 * t2680;
    let t18110 = t5599 * t762;
    let t18113 = t5605 * t2689;
    let t18119 = t5614 * t962;
    let t18122 = t1723 * t2650 / 6912.0;
    let t18133 = t5624 * t219;
    let t18142 = t5628 * t5570;
    (t18104, t18107, t18110, t18113, t18119, t18122, t18133, t18142)
}
