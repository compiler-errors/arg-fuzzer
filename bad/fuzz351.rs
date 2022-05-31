
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo351(_: S7, _: S3) {}
        
        fn test351() { foo351(S8, S6, S7, S1, S8, S5); }
    