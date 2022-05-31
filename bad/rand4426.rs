
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4426(_: S1, _: S2, _: S5, _: S6, _: S8) {}
        
        fn test4426() { foo4426(S0, S5, S1, S3, S5, S3, S2); }
    