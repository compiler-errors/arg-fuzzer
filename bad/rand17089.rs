
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo17089(_: S1, _: S5, _: S7, _: S8) {}
        
        fn test17089() { foo17089(S6, S6, S2, S3, S0, S0, S6); }
    