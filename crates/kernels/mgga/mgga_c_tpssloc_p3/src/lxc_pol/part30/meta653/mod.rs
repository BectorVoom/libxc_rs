//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta653 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2068;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2069;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta653<F: Float>(t25471: F, t82431: F, t7607: F, t82632: F, t25490: F, t82514: F, t3030: F, t343: F, t25483: F, t25486: F, t25492: F, t23478: F, t4547: F, t82573: F, t1920: F, t25766: F, t968: F, t23384: F, t25739: F, t25751: F, t4657: F, t6703: F, t7554: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t89445, t89449, t89468, t89501, t89505, t89532) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2068::<F>(t25471, t82431, t7607, t82632, t25490, t82514, t3030, t343, t25483, t25486, t25492, t23478, t4547);
        let (t89546, t89561, t89583, t89597, t89598, t89609) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2069::<F>(t7607, t82573, t1920, t25766, t968, t23384, t25739, t25751, t82431, t4657, t6703, t7554);
    (t89445, t89449, t89468, t89501, t89505, t89532, t89546, t89561, t89583, t89597, t89598, t89609)
}
