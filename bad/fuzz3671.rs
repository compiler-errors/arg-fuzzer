
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3671(_: S6, _: S3, _: S2, _: S5) {}
        
        fn test3671() { foo3671(S4, S6, S8, S5, S1); }
    