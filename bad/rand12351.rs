
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12351(_: S2, _: S5, _: S6, _: S7, _: S8) {}
        
        fn test12351() { foo12351(S2, S3, S2, S6, S4, S1, S6); }
    