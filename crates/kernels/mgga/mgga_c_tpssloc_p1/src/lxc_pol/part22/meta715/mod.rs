//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta715 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2321;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2322;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2323;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta715<F: Float>(t40741: F, t40743: F, t40748: F, t40760: F, t40764: F, t40766: F, t46292: F, t67162: F, t67163: F, t67169: F, t67170: F, t67174: F, t67176: F, t67178: F, t67180: F, t67183: F, t67186: F, t39529: F, t40779: F, t40784: F, t40790: F, t40793: F, t40797: F, t40799: F, t46311: F, t67191: F, t67204: F, t67206: F, t67207: F, t67210: F, t67211: F, t67212: F, t67214: F, t67215: F, t46447: F, t5499: F, t58972: F, t12939: F, t17635: F, t4195: F, t20217: F, t707: F, t751: F, t1462: F, t58976: F, t39549: F, t39563: F, t39585: F, t39590: F, t40801: F, t40803: F, t67216: F, t67217: F, t67226: F, t67228: F, t67231: F, t67244: F) -> (F, F, F, F, F, F, F, F) {
        let t67452 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2321::<F>(t40741, t40743, t40748, t40760, t40764, t40766, t46292, t67162, t67163, t67169, t67170, t67174, t67176, t67178, t67180, t67183, t67186);
        let t67455 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2322::<F>(t39529, t40779, t40784, t40790, t40793, t40797, t40799, t46311, t67191, t67204, t67206, t67207, t67210, t67211, t67212, t67214, t67215);
        let (t67457, t67458, t67461, t67464, t67466, t67467) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2323::<F>(t46447, t5499, t58972, t12939, t17635, t4195, t20217, t707, t751, t1462, t58976, t39549, t39563, t39585, t39590, t40801, t40803, t67216, t67217, t67226, t67228, t67231, t67244);
    (t67452, t67455, t67457, t67458, t67461, t67464, t67466, t67467)
}
