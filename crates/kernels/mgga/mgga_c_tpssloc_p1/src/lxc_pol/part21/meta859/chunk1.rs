//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3118/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3118<F: Float>(t18934: F, t3411: F, t1164: F, t4882: F, t51613: F, t18274: F, t3404: F, t300: F, t63709: F, t63290: F, t64475: F, t64477: F, t64479: F, t64481: F, t64485: F, t64489: F, t64492: F, t64496: F, t64499: F) -> (F, F, F, F, F) {
    let t64501 = F::cast_from(0.23392894490538584828e1_f64) * t3411 * t18934;
    let t64504 = F::cast_from(0.34631718211362927518e2_f64) * t1164 * t4882 * t51613;
    let t64507 = F::cast_from(0.6233709278045326953e3_f64) * t1164 * t18274 * t3404;
    let t64509 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t63709;
    let t64510 = t64475 - t64477 - t64479 - t64481 + t64485 + t64489 + t64492 - t64496 - t64499 + t64501 - t63290 - t64504 - t64507 + t64509;
    (t64501, t64504, t64507, t64509, t64510)
}
