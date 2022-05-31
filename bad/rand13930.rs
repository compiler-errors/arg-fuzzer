
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo13930(_: S0, _: S5, _: S7) {}
        
        fn test13930() { foo13930(S3, S4, S5, S8); }
    