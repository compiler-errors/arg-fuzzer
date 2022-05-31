
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo526(_: S2, _: S4) {}
        
        fn test526() { foo526(S1, S2, S3, S6, S7); }
    