//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1090/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1090<F: Float>(t32734: F, t32780: F, t533: F, t1390: F, t1983: F, t1442: F, t1849: F, t1869: F, t1976: F, t32656: F, t32659: F, t32661: F, t32664: F, t32666: F, t32668: F, t32671: F, t32674: F, t32676: F, t32679: F, t32680: F, t32684: F, t6517: F, t652: F, t7451: F, t7472: F, t7670: F, t8329: F, t8439: F, t8447: F) -> (F, F, F, F) {
    let t32781 = t32734 + t32780;
    let t32782 = t533 * t32781;
    let t32783 = t32782 * t1390;
    let t32784 = t1983 * t32783;
    let t32785 = -t1442 * t8439 + t1849 * t8447 - F::new(2.0) * t1869 * t7670 - F::new(2.0) * t1976 * t7451 - F::new(2.0) * t32656 * t652 - F::new(4.0) * t6517 * t7472 - F::new(4.0) * t32659 - F::new(4.0) * t32661 - F::new(4.0) * t32664 - t32666 + F::new(6.0) * t32668 - F::new(4.0) * t32671 - t32674 - t32676 - t32679 - F::new(4.0) * t32680 + t32684 + t32784 - t8329;
    (t32781, t32782, t32783, t32785)
}
