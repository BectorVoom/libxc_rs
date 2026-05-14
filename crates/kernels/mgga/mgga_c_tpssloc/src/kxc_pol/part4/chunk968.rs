//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 968/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk968<F: Float>(t3131: F, t4649: F, t4593: F, t4582: F, t16558: F, t998: F, t974: F, t13835: F, t4531: F, t13769: F, t13839: F, t1539: F, t6733: F, t4540: F, t7577: F, t4546: F) -> (F, F, F, F, F, F) {
    let t17732 = t3131 * t4649;
    let t17733 = t4593 * t17732;
    let t17734 = t4582 * t17733;
    let t17737 = t998 * t16558;
    let t17738 = t974 * t17737;
    let t17742 = t4531 * t13835;
    let t17745 = t13769 * t13839;
    let t17748 = t6733 * t1539;
    let t17749 = t4531 * t17748;
    let t17752 = t7577 * t4540;
    let t17753 = t4546 * t17752;
    (t17734, t17738, t17742, t17745, t17749, t17753)
}
