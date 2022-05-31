
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6172(_: S5, _: S6, _: S7) {}
        
        fn test6172() { foo6172(S4, S2, S2, S6, S3, S1, S6); }
    