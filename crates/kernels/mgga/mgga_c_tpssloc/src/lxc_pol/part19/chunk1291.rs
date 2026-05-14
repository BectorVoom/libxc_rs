//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1291/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1291<F: Float>(t39267: F, t404: F, t410: F, t3271: F, t1100: F, t43832: F, t3279: F, t3270: F, t407: F, t3287: F, t43855: F, t43857: F, t43859: F, t43861: F, t43863: F, t43866: F, t43869: F, t43872: F, t43875: F) -> (F, F, F, F, F, F) {
    let t43880 = 1.0 / t410 / t39267 / t404 / 96.0;
    let t43881 = t3271 * t3271;
    let t43882 = t43880 * t43881;
    let t43884 = t1100 * t43832;
    let t43886 = t3279 * t3279;
    let t43887 = t3270 * t43886;
    let t43889 = f64::powf(t407, -0.25e1);
    let t43890 = t43889 * t43881;
    let t43892 = t3287 * t43886;
    let t43894 = -0.18396666666666666667e0 * t43855 - 0.98115555555555555555e-1 * t43857 - 0.98115555555555555556e0 * t43859 + 0.5519e0 * t43861 + 0.11038e1 * t43863 - 0.51785e1 * t43866 + 0.3300975e0 * t43869 + 0.11651625e2 * t43872 - 0.247573125e0 * t43875 - 0.485484375e1 * t43882 + 0.258925e1 * t43884 - 0.3883875e1 * t43887 + 0.6189328125e-1 * t43890 + 0.247573125e0 * t43892;
    (t43882, t43884, t43887, t43890, t43892, t43894)
}
