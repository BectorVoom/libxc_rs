//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 917/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk917<F: Float>(t128293: F, t128388: F, t128433: F, t128469: F, t128514: F, t128553: F, t128593: F, t128973: F, t27254: F, t7467: F, t100996: F, t1873: F, t100911: F, t115984: F, t122811: F, t127608: F, t127627: F, t127646: F, t127647: F, t127698: F, t127701: F, t127706: F, t127708: F, t127714: F, t1458: F, t2039: F, t23880: F, t28951: F, t29422: F, t29425: F, t5456: F, t577: F, t7010: F) -> (F, F) {
    let t128976 = t128293 + t128388 + t128433 + t128469 + t128514 + t128553 + t128593 + t128973;
    let t128984 = 27.0 * t27254 * t7467;
    let t128988 = 0.135e2 * t100996 * t1873;
    let t128989 = t127698 + t127701 + 0.135e2 * t7010 * t28951 + t127608 + t127706 + t127708 + 54.0 * t23880 * t29422 + 27.0 * t23880 * t29425 + t127714 + 27.0 * t115984 * t5456 + t127627 + 0.45e1 * t128976 * t577 + 27.0 * t122811 * t1458 + 27.0 * t127647 * t2039 + t127646 + t128984 + 0.135e2 * t100911 * t2039 + t128988;
    (t128976, t128989)
}
