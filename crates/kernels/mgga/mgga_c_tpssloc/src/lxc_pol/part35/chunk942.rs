//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 942/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk942<F: Float>(t10608: F, t13598: F, t17149: F, t17165: F, t17175: F, t21124: F, t21128: F, t21147: F, t21150: F, t21153: F, t21156: F, t324: F, t10832: F, t14276: F, t21259: F, t21263: F, t21265: F, t21267: F, t21270: F, t21302: F, t21305: F, t21306: F, t21309: F, t21312: F, t21317: F, t21320: F, t21321: F, t21336: F, t2861: F, t2886: F, t2905: F, t2930: F, t311: F, t5743: F) -> (F, F) {
    let t21347 = -t10608 - 0.12361111111111111111e-1 * t13598 + 0.61805555555555555556e-2 * t17149 - 0.18541666666666666667e-1 * t17165 + 0.92708333333333333334e-2 * t17175 - 0.10300925925925925926e-1 * t21147 + 0.37083333333333333333e-1 * t21150 - 0.18541666666666666666e-1 * t21124 - 0.55625000000000000001e-1 * t21153 + 0.55625000000000000001e-1 * t21128 - 0.92708333333333333333e-2 * t21156;
    let t21348 = t21347 * t324;
    let t21360 = -t10832 - 0.2283111111111111111e-1 * t13598 + 0.11415555555555555555e-1 * t17149 - 0.34246666666666666665e-1 * t17165 + 0.17123333333333333333e-1 * t17175 - 0.19025925925925925925e-1 * t21147 + 0.68493333333333333331e-1 * t21150 - 0.34246666666666666665e-1 * t21124 - 0.10274e0 * t21153 + 0.10274e0 * t21128 - 0.17123333333333333333e-1 * t21156;
    let t21363 = -6.0 * t14276 * t5743 + 6.0 * t2886 * t21259 - t21263 - t21265 - t21267 + t21270 - t21302 - t21305 + 0.96491876992155210402e2 * t2886 * t21306 - 0.35089341735807877242e1 * t2905 * t21309 + 0.51947577317044391277e2 * t2930 * t21312 + t21317 - t21320 - 6.0 * t2861 * t21321 + t21336 - 0.19751673498613801407e-1 * t21348 - 0.310907e-1 * t21360 * t311;
    (t21348, t21363)
}
