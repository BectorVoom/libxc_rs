//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1147/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1147<F: Float>(t39706: F, t39749: F, t39803: F, t39840: F, t17: F, t521: F, t2225: F, t3826: F, t193: F, t23857: F, t3701: F, t3914: F, t39629: F, t39631: F, t39633: F, t39635: F, t39637: F, t39640: F, t39643: F, t39645: F, t39649: F, t39655: F, t39658: F, t39660: F, t5160: F, t533: F) -> (F, F, F, F) {
    let t39842 = t39706 + t39749 + t39803 + t39840;
    let t39844 = t17 * t521 * t39842;
    let t39845 = t2225 * t3826;
    let t39846 = F::new(240.0) * t39845;
    let t39847 = -F::new(3.0) * t193 * t3701 * t39649 * t533 + F::new(12.0) * t23857 * t3914 * t5160 + t39629 + t39631 - t39633 + t39635 + t39637 + t39640 + t39643 - t39645 - t39655 - t39658 - t39660 + t39844 + t39846;
    (t39842, t39844, t39846, t39847)
}
