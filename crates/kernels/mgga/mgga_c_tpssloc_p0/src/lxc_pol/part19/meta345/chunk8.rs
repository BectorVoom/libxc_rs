//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1242/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1242<F: Float>(t225: F, t9520: F, t10049: F, t10054: F, t10055: F, t10076: F, t10084: F, t10097: F, t10101: F, t10103: F, t10104: F, t10112: F, t10116: F, t218: F, t22997: F, t23175: F, t252: F, t259: F, t2597: F, t2617: F, t2633: F, t2679: F, t2684: F, t2718: F, t2720: F, t2729: F, t2733: F, t2736: F, t2738: F, t2743: F, t40890: F, t40891: F, t40895: F, t40904: F, t40909: F, t40917: F, t41230: F, t41388: F, t41490: F, t41495: F, t41520: F, t41549: F, t4182: F, t4281: F, t4291: F, t812: F, t829: F, t852: F, t855: F, t858: F, t860: F, t861: F, t865: F, t866: F, t9584: F, t9590: F, t9612: F, t9632: F, t9976: F, t9981: F) -> F {
    let t41554 = t9520 * t225;
    let t41580 = F::new(24.0) * t855 * t40890 * t40891 - t855 * t858 * (F::new(24.0) * t812 * t10054 * t9981 - F::new(6.0) * t812 * t10076 * t2684 + F::new(12.0) * t812 * t40895 * t2633 + F::new(8.0) * t4281 * t40909 * t4182 - F::new(24.0) * t812 * t40917 * t9976 + F::new(24.0) * t2617 * t10055 + F::new(24.0) * t2617 * t10084 + F::new(12.0) * t9612 * t2729 - F::new(6.0) * t9612 * t2736 - F::new(4.0) * t40904 * t861 + t41495 - F::new(6.0) * t812 * t10076 * t2679 - F::new(6.0) * t4291 * t10097 * t2684 + F::new(36.0) * t4281 * t22997 * t9632 + F::new(24.0) * t4281 * t23175 * t9632 - F::new(4.0) * t4291 * t40909 * t829 - t812 * t860 * t41388 - F::new(4.0) * t812 * t41520 * t829 - F::new(4.0) * t2617 * t10101 - F::new(12.0) * t9612 * t2733 - F::new(6.0) * t9612 * t2738 + t41549) - F::new(12.0) * t41554 * t866 - F::new(24.0) * t2597 * t10112 + F::new(24.0) * t2597 * t10116 + F::new(8.0) * t855 * t2718 * t10103 * t865 - F::new(6.0) * t9590 * t2743 - F::new(4.0) * t2597 * t10104 - F::new(6.0) * t10049 * t2743 + t41230 * t252 * t259 + F::new(12.0) * t9590 * t2720 + F::new(4.0) * t9584 * t852 * t259 + t218 * t41490 * t259;
    t41580
}
