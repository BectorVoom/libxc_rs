//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta859 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3117;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3118;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta859<F: Float>(t1157: F, t1164: F, t18785: F, t3375: F, t18279: F, t3378: F, t1147: F, t1156: F, t64425: F, t15225: F, t51819: F, t64482: F, t18934: F, t3411: F, t4882: F, t51613: F, t18274: F, t3404: F, t300: F, t63709: F, t63290: F, t64475: F, t64477: F, t64479: F, t64481: F, t64485: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t64489, t64492, t64496, t64499) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3117::<F>(t1157, t1164, t18785, t3375, t18279, t3378, t1147, t1156, t64425, t15225, t51819, t64482);
        let (t64501, t64504, t64507, t64509, t64510) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3118::<F>(t18934, t3411, t1164, t4882, t51613, t18274, t3404, t300, t63709, t63290, t64475, t64477, t64479, t64481, t64485, t64489, t64492, t64496, t64499);
    (t64489, t64492, t64496, t64499, t64501, t64504, t64507, t64509, t64510)
}
