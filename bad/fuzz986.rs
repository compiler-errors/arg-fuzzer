
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo986(_: S1, _: S5) {}
        
        fn test986() { foo986(S7, S8, S1, S8); }
    