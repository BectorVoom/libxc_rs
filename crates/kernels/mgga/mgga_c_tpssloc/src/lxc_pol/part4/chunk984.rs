//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 984/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk984<F: Float>(t4571: F, t4644: F, t1031: F, t5904: F, t1022: F, t1539: F, t14211: F, t3071: F, t1023: F, t5685: F, t1616: F, t4343: F, t1009: F, t5848: F, t1011: F, t1019: F) -> (F, F, F, F, F, F, F) {
    let t18008 = t4644 * t4571;
    let t18010 = t5904 * t1031;
    let t18014 = t1539 * t1022;
    let t18015 = t14211 * t18014;
    let t18016 = t3071 * t18015;
    let t18020 = t5685 * t1023;
    let t18021 = t3071 * t18020;
    let t18024 = t1616 * t4343;
    let t18025 = t3071 * t18024;
    let t18028 = t5848 * t1009;
    let t18029 = t18028 * t1011;
    let t18030 = t18029 * t1019;
    (t18008, t18010, t18016, t18021, t18025, t18028, t18030)
}
