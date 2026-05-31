//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1299/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1299<F: Float>(t5664: F, t67159: F, t58021: F, t46278: F, t67177: F, t1484: F, t1530: F, t1877: F, t193: F, t202: F, t39483: F, t40741: F, t40743: F, t40748: F, t40760: F, t40764: F, t40766: F, t40772: F, t4314: F, t67154: F, t67235: F) -> (F, F, F, F, F) {
    let t75879 = t5664 * t5664;
    let t75884 = F::cast_from(4.0_f64) * t67159;
    let t75885 = F::cast_from(0.35089341735807877242e1_f64) * t58021;
    let t75886 = F::cast_from(0.65061487801810439052e-1_f64) * t46278;
    let t75887 = F::cast_from(48.0_f64) * t67177;
    let t75891 = -F::cast_from(6.0_f64) * t193 * t202 * t40772 * t75879 + F::cast_from(24.0_f64) * t1484 * t4314 * t67235 - F::cast_from(4.0_f64) * t1530 * t1877 * t67154 + t39483 - t40741 - t40743 + t40748 + t40760 + t40764 + t40766 + t75884 - t75885 + t75886 + t75887;
    (t75884, t75885, t75886, t75887, t75891)
}
