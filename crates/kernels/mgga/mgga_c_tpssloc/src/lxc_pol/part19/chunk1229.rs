//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1229/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1229<F: Float>(t10209: F, t10217: F, t10263: F, t10325: F, t2960: F, t2979: F, t3000: F, t343: F, t39097: F, t42554: F, t42861: F, t42862: F, t42873: F, t42877: F, t42889: F, t42893: F, t42895: F, t4546: F, t973: F, t980: F, t984: F, t987: F) -> (F,) {
    let t42899 = 0.28806584362139917695e-2 * t973 * t42861 * t42862 * t39097 - 0.33333333333333333332e-2 * t973 * t4546 * t10325 * t984 * t343 - 0.37037037037037037036e-3 * t42873 - 0.49382716049382716048e-3 * t42877 + 0.16296296296296296296e-1 * t10263 * t3000 + 0.26666666666666666666e-1 * t2960 * t10209 + 0.13333333333333333332e-1 * t973 * t2979 * t10217 * t39097 - 0.50699588477366255142e-1 * t42554 * t980 + 0.1086419753086419753e-1 * t42889 + 0.41152263374485596707e-3 * t42893 - 0.1086419753086419753e-1 * t42895 + 0.15209876543209876543e0 * t42554 * t987;
    (t42899,)
}
