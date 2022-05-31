
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11704(_: S1, _: S7, _: S8) {}
        
        fn test11704() { foo11704(S7, S6, S7, S2, S1, S0, S6); }
    