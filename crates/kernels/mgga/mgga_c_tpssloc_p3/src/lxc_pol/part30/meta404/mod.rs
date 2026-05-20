//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1539;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1540;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta404<F: Float>(t10523: F, t5774: F, t4497: F, t959: F, t4472: F, t4488: F, t2929: F, t5790: F, t17490: F, t17504: F, t17506: F, t17509: F, t17512: F, t17515: F, t17519: F, t17523: F, t17526: F, t17530: F, t17936: F, t17940: F, t17942: F, t17944: F, t17946: F, t17933: F, t360: F, t1021: F, t248: F, t1020: F, t10413: F, t10891: F, t10949: F, t14077: F, t14080: F, t14136: F, t14139: F, t14207: F, t1618: F, t1622: F, t17907: F, t17920: F, t17925: F, t3048: F, t3070: F, t4641: F, t4652: F, t5857: F, t5875: F, t5880: F, t5900: F) -> (F, F, F, F, F, F) {
        let (t17950, t17953, t17957, t17958) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1539::<F>(t10523, t5774, t4497, t959, t4472, t4488, t2929, t5790, t17490, t17504, t17506, t17509, t17512, t17515, t17519, t17523, t17526, t17530, t17936, t17940, t17942, t17944, t17946);
        let (t17959, t17962, t17967) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1540::<F>(t17933, t17958, t360, t1021, t248, t1020, t10413, t10891, t10949, t14077, t14080, t14136, t14139, t14207, t1618, t1622, t17907, t17920, t17925, t3048, t3070, t4641, t4652, t5857, t5875, t5880, t5900);
    (t17950, t17953, t17957, t17959, t17962, t17967)
}
