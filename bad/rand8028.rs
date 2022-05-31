
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8028(_: S4, _: S5, _: S3) {}
        
        fn test8028() { foo8028(S1, S4, S6, S6); }
    