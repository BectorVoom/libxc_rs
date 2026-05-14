//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1326/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1326<F: Float>(t3502: F, t42341: F, t44696: F, t23508: F, t3508: F, t225: F, t44657: F, t1209: F, t475: F, t43670: F, t43672: F, t43674: F, t43678: F, t43683: F, t43685: F, t43687: F, t43695: F, t43702: F, t43915: F, t43924: F) -> (F, F, F, F, F, F) {
    let t44753 = t44696 * t42341 * t3502;
    let t44754 = t23508 * t3508;
    let t44774 = t44657 * t225;
    let t44785 = t44696 * t42341 * t1209;
    let t44786 = t23508 * t475;
    let t44792 = -t43670 - t43672 + t43674 - t43678 - t43683 + t43685 - t43687 - t43695 - t43702 - t43915 + t43924;
    (t44753, t44754, t44774, t44785, t44786, t44792)
}
