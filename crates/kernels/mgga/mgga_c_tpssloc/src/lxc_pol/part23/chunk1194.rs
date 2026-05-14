//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1194/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1194<F: Float>(t39529: F, t40764: F, t40766: F, t40779: F, t40784: F, t75894: F, t75895: F, t75900: F, t75901: F, t75932: F, t75933: F, t39549: F, t39563: F, t40790: F, t40793: F, t40797: F, t40799: F, t40801: F, t40803: F, t75939: F, t75940: F, t75941: F, t75942: F) -> (F, F) {
    let t76013 = t40764 + t40766 + t75894 + t75895 - t39529 + t75900 - t75901 - t40779 + t75932 + t40784 + t75933;
    let t76014 = t40790 + t40793 + t40797 + t40799 + t40801 - t40803 + t39549 + t75939 + t39563 + t75940 + t75941 + t75942;
    (t76013, t76014)
}
