//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta401 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1526;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1527;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1528;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1529;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1530;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta401<F: Float>(t3131: F, t4649: F, t4593: F, t4582: F, t16558: F, t998: F, t974: F, t13835: F, t4531: F, t13769: F, t13839: F, t1539: F, t6733: F, t4540: F, t7577: F, t4546: F, t343: F, t5842: F, t984: F, t2970: F, t5824: F, t973: F, t10226: F, t13782: F, t13787: F, t13790: F, t13825: F, t2960: F, t2986: F, t5825: F, t5828: F, t978: F, t977: F, t5836: F, t10231: F, t5817: F, t13861: F, t17178: F, t4510: F, t2989: F, t5398: F, t2988: F, t10186: F, t13830: F, t13850: F, t5818: F, t5821: F, t5829: F, t2987: F, t2990: F, t13847: F, t4514: F, t17167: F, t4518: F, t17171: F, t10254: F, t5392: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17732, t17734, t17738, t17742, t17745, t17748) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1526::<F>(t3131, t4649, t4593, t4582, t16558, t998, t974, t13835, t4531, t13769, t13839, t1539, t6733);
        let t17766 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1527::<F>(t17748, t4531, t4540, t7577, t4546, t343, t5842, t984, t2970, t5824, t973, t10226, t13782, t13787, t13790, t13825, t17742, t17745, t2960, t2986, t5825);
        let (t17770, t17773, t17778, t17783) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1528::<F>(t2970, t5828, t973, t16558, t978, t977, t343, t5836, t984, t4546, t10231, t5817);
        let t17798 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1529::<F>(t17783, t973, t13861, t4531, t17178, t4510, t2989, t5398, t2988, t10186, t13830, t13850, t17770, t17773, t17778, t2960, t2986, t5818, t5821, t5829);
        let (t17801, t17805, t17809, t17811, t17814, t17817) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1530::<F>(t2987, t5836, t2990, t5842, t13847, t4514, t2986, t17167, t4518, t17171, t10254, t5392);
    (t17732, t17734, t17738, t17766, t17798, t17801, t17805, t17809, t17811, t17814, t17817)
}
