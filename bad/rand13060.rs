
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo13060(_: S8, _: S7) {}
        
        fn test13060() { foo13060(S5, S8, S6); }
    