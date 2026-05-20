//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1408;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1409;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1410;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1411;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1412;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta472<F: Float>(t123: F, t3240: F, t77965: F, t63332: F, t63334: F, t63888: F, t63893: F, t63911: F, t71142: F, t71144: F, t71146: F, t71152: F, t71154: F, t71156: F, t71408: F, t78002: F, t5999: F, t3270: F, t5992: F, t43889: F, t1409: F, t71137: F, t18205: F, t5398: F, t11145: F, t18210: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t78005 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1408::<F>(t123, t3240, t77965);
        let t78019 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1409::<F>(t63332, t63334, t63888, t63893, t63911, t71142, t71144, t71146, t71152, t71154, t71156, t71408, t78002, t78005);
        let (t78025, t78026, t78028, t78029, t78031, t78033) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1410::<F>(t5999, t3270, t5992, t43889, t1409, t71137, t123, t3240);
        let (t78035, t78037) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1411::<F>(t18205, t5398, t11145, t123);
        let (t78039, t78041) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1412::<F>(t18210, t5398, t123, t3240);
    (t78005, t78019, t78025, t78026, t78028, t78029, t78031, t78033, t78035, t78037, t78039, t78041)
}
