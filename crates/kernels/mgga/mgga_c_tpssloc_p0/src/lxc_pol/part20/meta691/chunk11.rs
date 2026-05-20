//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2634/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2634<F: Float>(t11913: F, t52834: F, t11880: F, t11712: F, t11887: F, t491: F, t11638: F, t11871: F, t11877: F, t11897: F, t11904: F, t1244: F, t1246: F, t14997: F, t15022: F, t15032: F, t15430: F, t15777: F, t1755: F, t1932: F, t3493: F, t3604: F, t3610: F, t3621: F, t3624: F, t45329: F, t475: F, t5052: F, t5064: F, t5083: F, t5084: F, t52480: F, t52709: F) -> (F, F, F) {
    let t53592 = t52834 * t11913;
    let t53613 = t52834 * t11880;
    let t53646 = t11712 * t11887 * t491;
    let t53650 = -t11638 * t1755 * t1932 * t3624 * t475 + F::new(3.0) * t1244 * t1246 * t3493 * t5052 + F::new(6.0) * t11871 * t3610 * t5083 - F::new(3.0) * t15022 * t3624 * t5083 - F::new(18.0) * t52480 * t52709 * t53646 + F::new(3.0) * t11877 * t5084 + F::new(3.0) * t11897 * t5064 + F::new(12.0) * t11904 * t14997 + F::new(3.0) * t15032 * t3621 + F::new(3.0) * t15430 * t45329 + F::new(6.0) * t15777 * t3604;
    (t53592, t53613, t53650)
}
