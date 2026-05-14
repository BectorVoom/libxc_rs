//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1166/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1166<F: Float>(t11678: F, t11697: F, t22279: F, t22161: F, t3577: F, t19025: F, t5001: F, t22243: F, t486: F, t1222: F, t22116: F, t18332: F, t4889: F, t22153: F, t13969: F, t22274: F, t3515: F) -> (F, F, F, F, F, F, F, F) {
    let t72936 = t11678 * t11697 * t22279;
    let t72959 = t3577 * t11697 * t22161;
    let t72967 = t5001 * t19025;
    let t73028 = t486 * t22243;
    let t73043 = t22116 * t1222;
    let t73076 = t4889 * t18332;
    let t73084 = t3577 * t11697 * t22153;
    let t73096 = t3515 * t13969 * t22274;
    (t72936, t72959, t72967, t73028, t73043, t73076, t73084, t73096)
}
