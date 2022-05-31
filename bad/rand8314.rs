
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8314(_: S3, _: S4) {}
        
        fn test8314() { foo8314(S1, S2, S3, S5, S7); }
    