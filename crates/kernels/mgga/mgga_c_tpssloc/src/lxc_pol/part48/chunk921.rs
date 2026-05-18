//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 921/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk921<F: Float>(t2006: F, t212: F, t22642: F, t6890: F, t3886: F, t6992: F, t1385: F, t1992: F, t22635: F, t1985: F, t22904: F, t6889: F, t6906: F) -> (F, F, F) {
    let t113941 = F::new(0.16449340668482264365e-1) * t22642 * t212 * t2006 * t6890;
    let t113946 = t3886 * t6992;
    let t113950 = F::new(0.6579736267392905746e-1) * t1992 * t22635 * t113946 * t1385;
    let t113956 = F::new(0.16449340668482264365e-1) * t1985 * t6889 * t6906 * t22904;
    (t113941, t113950, t113956)
}
