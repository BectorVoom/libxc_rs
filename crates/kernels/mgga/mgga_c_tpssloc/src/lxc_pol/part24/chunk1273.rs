//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1273/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1273<F: Float>(t1992: F, t550: F, t6976: F, t81028: F, t22863: F, t6979: F, t12267: F, t1336: F, t22873: F, t22877: F, t3773: F, t3777: F, t3851: F, t544: F, t553: F, t6988: F, t6990: F, t81011: F, t81016: F, t81019: F, t81022: F, t81031: F, t81037: F, t81039: F, t81041: F, t81043: F, t81047: F, t81050: F, t81055: F) -> F {
    let t81059 = t1992 * t6976 * t81028 * t550;
    let t81061 = t22863 * t6979;
    let t81063 = F::new(3.0) * t3773 * t6990 + t544 * t553 * t81011 + F::cast_from(0.49348022005446793095e-1_f64) * t81016 + F::cast_from(0.49348022005446793095e-1_f64) * t81019 - F::cast_from(0.24674011002723396548e-1_f64) * t81022 - F::new(3.0) * t1336 * t22873 * t3851 - F::cast_from(0.49348022005446793095e-1_f64) * t81031 - F::new(3.0) * t12267 * t6988 - F::new(3.0) * t3777 * t22877 - F::cast_from(0.57572692339687925277e-1_f64) * t81037 + F::cast_from(0.19190897446562641759e0_f64) * t81039 + F::cast_from(0.57572692339687925277e-1_f64) * t81041 - F::cast_from(0.34543615403812755166e0_f64) * t81043 - F::cast_from(0.78134368175290755733e-1_f64) * t81047 + F::cast_from(0.24674011002723396547e-1_f64) * t81050 + F::cast_from(0.14804406601634037928e0_f64) * t81055 - F::cast_from(0.82246703342411321825e-2_f64) * t81059 - F::cast_from(0.19190897446562641759e0_f64) * t81061;
    t81063
}
