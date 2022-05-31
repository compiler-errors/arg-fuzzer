
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3930(_: S2, _: S3, _: S1) {}
        
        fn test3930() { foo3930(S8, S7, S1, S2, S8, S6); }
    