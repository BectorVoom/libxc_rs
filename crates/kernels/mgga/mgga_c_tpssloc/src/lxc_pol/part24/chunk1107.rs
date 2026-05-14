//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1107/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1107<F: Float>(t23620: F, t345: F, t6680: F, t6781: F, t6805: F, t968: F, t1920: F, t210: F, t6795: F, t6688: F, t974: F, t381: F, t883: F, t6743: F, t14227: F, t6800: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t23621 = t345 * t23620;
    let t23626 = t6680 * t6781;
    let t23628 = t968 * t6805;
    let t23629 = t1920 * t23628;
    let t23631 = t6795 * t210;
    let t23632 = t974 * t6688;
    let t23633 = t23631 * t23632;
    let t23634 = t381 * t883;
    let t23635 = t6743 * t23634;
    let t23636 = t14227 * t6800;
    (t23621, t23626, t23628, t23629, t23631, t23632, t23633, t23634, t23635, t23636)
}
