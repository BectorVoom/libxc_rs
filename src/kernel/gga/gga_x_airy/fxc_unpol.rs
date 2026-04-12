//! GGA_X_AIRY fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_airy.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_airy_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = t20 * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = 1.0 / t23;
        let t25 = t21 * t24;
        let t26 = f64::sqrt(sigma[ip]);
        let t27 = M_CBRT2;
        let t28 = t26 * t27;
        let t30 = 1.0 / t18 / rho[ip];
        let t32 = t25 * t28 * t30;
        let t33 = f64::powf(t32, 0.2626712e1);
        let t35 = 1.0 + 0.13471619689594796103e-3 * t33;
        let t36 = f64::powf(t35, -0.657946e0);
        let t39 = f64::powf(t32, 0.3217063e1);
        let t41 = f64::powf(t32, 0.3223476e1);
        let t43 = 1.0 - 0.45212413010769857073e-1 * t39 + 0.45402221956620378581e-1 * t41;
        let t44 = f64::powf(t32, 0.3473804e1);
        let t46 = 1.0 + 0.47702180224903349918e-3 * t44;
        let t47 = 1.0 / t46;
        let t49 = 0.60146019220211109872e-4 * t33 * t36 + t43 * t47;
        let t53 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t49);
        let tzk0 = 2.0 * t53;
        zk[ip] += tzk0;
        let t54 = t18 * t18;
        let t56 = t17 / t54;
        let t60 = f64::powf(t32, 0.1626712e1);
        let t62 = t60 * t36 * t21;
        let t63 = t24 * t26;
        let t64 = rho[ip] * rho[ip];
        let t66 = 1.0 / t18 / t64;
        let t67 = t27 * t66;
        let t68 = t63 * t67;
        let t71 = f64::powf(t32, 0.4253424e1);
        let t72 = f64::powf(t35, -0.1657946e1);
        let t74 = t71 * t72 * t21;
        let t77 = f64::powf(t32, 0.2217063e1);
        let t79 = t77 * t21 * t24;
        let t80 = t28 * t66;
        let t83 = f64::powf(t32, 0.2223476e1);
        let t85 = t83 * t21 * t24;
        let t88 = 0.19393490805022174494e0 * t79 * t80 - 0.19513729709845177529e0 * t85 * t80;
        let t90 = t46 * t46;
        let t91 = 1.0 / t90;
        let t92 = t43 * t91;
        let t93 = f64::powf(t32, 0.2473804e1);
        let t94 = t93 * t21;
        let t95 = t92 * t94;
        let t98 = -0.21064836058394555311e-3 * t62 * t68 + 0.18671024483029835192e-7 * t74 * t68 + t88 * t47 + 0.22094403263198687541e-2 * t95 * t68;
        let t103 = piecewise3(t2, 0.0, -t6 * t56 * t49 / 8.0 - 3.0 / 8.0 * t6 * t19 * t98);
        let tvrho0 = 2.0 * rho[ip] * t103 + 2.0 * t53;
        vrho[ip] += tvrho0;
        let t106 = 1.0 / t26;
        let t107 = t24 * t106;
        let t108 = t27 * t30;
        let t109 = t107 * t108;
        let t114 = t106 * t27;
        let t115 = t114 * t30;
        let t120 = -0.72725590518833154352e-1 * t79 * t115 + 0.73176486411919415733e-1 * t85 * t115;
        let t124 = 0.78993135218979582417e-4 * t62 * t109 - 0.7001634181136188197e-8 * t74 * t109 + t120 * t47 - 0.82854012236995078279e-3 * t95 * t109;
        let t128 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t124);
        let tvsigma0 = 2.0 * rho[ip] * t128;
        vsigma[ip] += tvsigma0;
        let t133 = t17 / t54 / rho[ip];
        let t140 = f64::powf(t32, 0.626712e0);
        let t142 = t140 * t36 * t20;
        let t143 = t23 * t23;
        let t144 = 1.0 / t143;
        let t145 = t144 * sigma[ip];
        let t146 = t27 * t27;
        let t147 = t64 * t64;
        let t149 = 1.0 / t54 / t147;
        let t150 = t146 * t149;
        let t151 = t145 * t150;
        let t154 = f64::powf(t32, 0.3253424e1);
        let t156 = t154 * t72 * t20;
        let t159 = t64 * rho[ip];
        let t161 = 1.0 / t18 / t159;
        let t162 = t27 * t161;
        let t163 = t63 * t162;
        let t166 = f64::powf(t32, 0.5880136e1);
        let t167 = f64::powf(t35, -0.2657946e1);
        let t169 = t166 * t167 * t20;
        let t174 = f64::powf(t32, 0.1217063e1);
        let t175 = t174 * t20;
        let t176 = t175 * t144;
        let t177 = sigma[ip] * t146;
        let t178 = t177 * t149;
        let t181 = t28 * t161;
        let t184 = f64::powf(t32, 0.1223476e1);
        let t185 = t184 * t20;
        let t186 = t185 * t144;
        let t191 = -0.343972727237239018e1 * t176 * t178 - 0.45251478545051740486e0 * t79 * t181 + 0.34710647744262172761e1 * t186 * t178 + 0.45532035989638747568e0 * t85 * t181;
        let t193 = t88 * t91;
        let t194 = t193 * t94;
        let t198 = 1.0 / t90 / t46;
        let t199 = t43 * t198;
        let t200 = f64::powf(t32, 0.4947608e1);
        let t201 = t200 * t20;
        let t202 = t199 * t201;
        let t205 = f64::powf(t32, 0.1473804e1);
        let t206 = t205 * t20;
        let t207 = t92 * t206;
        let t212 = 0.27413137275378499087e-2 * t142 * t151 - 0.10276735016205996654e-5 * t156 * t151 + 0.49151284136253962392e-3 * t62 * t163 + 0.87631609607945203792e-10 * t169 * t151 - 0.43565723793736282115e-7 * t74 * t163 + t191 * t47 + 0.44188806526397375082e-2 * t194 * t68 + 0.58579518666821377499e-4 * t202 * t151 - 0.43725778536091172827e-1 * t207 * t151 - 0.51553607614130270929e-2 * t95 * t163;
        let t217 = piecewise3(t2, 0.0, t6 * t133 * t49 / 12.0 - t6 * t56 * t98 / 4.0 - 3.0 / 8.0 * t6 * t19 * t212);
        let tv2rho20 = 2.0 * rho[ip] * t217 + 4.0 * t103;
        v2rho2[ip] += tv2rho20;
        let t223 = t144 * t146;
        let t225 = 1.0 / t54 / t159;
        let t226 = t223 * t225;
        let t231 = t107 * t67;
        let t240 = t114 * t66;
        let t247 = 0.12898977271396463175e1 * t175 * t226 + 0.96967454025110872469e-1 * t79 * t240 - 0.13016492904098314785e1 * t185 * t226 - 0.97568648549225887644e-1 * t85 * t240;
        let t249 = t120 * t91;
        let t250 = t249 * t94;
        let t255 = t199 * t200;
        let t256 = t20 * t144;
        let t257 = t146 * t225;
        let t258 = t256 * t257;
        let t261 = t92 * t205;
        let t266 = -0.10279926478266937158e-2 * t142 * t226 + 0.38537756310772487454e-6 * t156 * t226 - 0.10532418029197277656e-3 * t62 * t231 - 0.32861853602979451422e-10 * t169 * t226 + 0.9335512241514917596e-8 * t74 * t231 + t247 * t47 + 0.22094403263198687541e-2 * t250 * t68 - 0.82854012236995078279e-3 * t194 * t109 - 0.21967319500058016562e-4 * t255 * t258 + 0.1639716695103418981e-1 * t261 * t258 + 0.11047201631599343771e-2 * t95 * t231;
        let t271 = piecewise3(t2, 0.0, -t6 * t56 * t124 / 8.0 - 3.0 / 8.0 * t6 * t19 * t266);
        let tv2rhosigma0 = 2.0 * rho[ip] * t271 + 2.0 * t128;
        v2rhosigma[ip] += tv2rhosigma0;
        let t274 = 1.0 / sigma[ip];
        let t275 = t144 * t274;
        let t277 = 1.0 / t54 / t64;
        let t278 = t146 * t277;
        let t279 = t275 * t278;
        let t284 = t26 * sigma[ip];
        let t285 = 1.0 / t284;
        let t286 = t24 * t285;
        let t287 = t286 * t108;
        let t294 = t274 * t146;
        let t295 = t294 * t277;
        let t298 = t285 * t27;
        let t299 = t298 * t30;
        let t306 = -0.48371164767736736906e0 * t176 * t295 + 0.36362795259416577176e-1 * t79 * t299 + 0.48811848390368680445e0 * t186 * t295 - 0.36588243205959707866e-1 * t85 * t299;
        let t316 = 0.38549724293501014342e-3 * t142 * t279 - 0.14451658616539682796e-6 * t156 * t279 - 0.39496567609489791208e-4 * t62 * t287 + 0.12323195101117294284e-10 * t169 * t279 + 0.35008170905680940985e-8 * t74 * t287 + t306 * t47 - 0.16570802447399015656e-2 * t250 * t109 + 0.82377448125217562105e-5 * t202 * t279 - 0.61489376066378211788e-2 * t207 * t279 + 0.4142700611849753914e-3 * t95 * t287;
        let t320 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t316);
        let tv2sigma20 = 2.0 * rho[ip] * t320;
        v2sigma2[ip] += tv2sigma20;
    }
}
